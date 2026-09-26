//! Self-only window capture for the pipe `screenshot` verb (`control_pipe.rs`).
//!
//! Deliberately takes an `HWND`, never a raw pointer/PID string from the wire: callers must go
//! through `control_pipe.rs`'s label lookup (`app.get_webview_window(label)`, restricted to
//! `crate::ALL_WINDOWS`), so this module can only ever be pointed at a window Moonpool itself
//! owns. There is no "capture any window" path here on purpose - see the safety discussion this
//! feature grew out of: `PrintWindow` renders a window's own content into an off-screen bitmap
//! rather than reading the screen, so it is not caught by OS/policy screen-capture restrictions
//! and it cannot pick up another app's pixels even if that app is drawn on top - unless the code
//! itself hands it someone else's HWND, which this API shape rules out.
//!
//! Output is PNG-encoded, not raw BMP: an uncompressed 32bpp capture of a full window is tens of
//! megabytes as base64 text, which blows past MCP/tool-result size limits. PNG compression brings
//! that down to a size small enough to hand back as an inline MCP image content block (see
//! `mcp.rs`), which a client like Claude Code renders directly - no manual decode step, no file
//! left on disk.

use std::io::BufWriter;

use windows::core::BOOL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, GetDC, ReleaseDC, SelectObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HDC, HGDIOBJ,
};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

/// Render into a bitmap without asking the window to erase its background first - avoids a
/// visible flash and matches what a normal repaint looks like.
const PW_RENDERFULLCONTENT: u32 = 0x00000002;

// `PrintWindow` (user32.dll) is absent from the `windows`/`windows-sys` crates' generated
// bindings (checked both at the pinned versions here) even though it is a plain, long-stable
// Win32 API - declared directly rather than pulling in a third crate for one function.
#[link(name = "user32")]
extern "system" {
    fn PrintWindow(hwnd: HWND, hdc_blt: HDC, flags: u32) -> BOOL;
}

/// Render `hwnd`'s own content (not whatever is on top of it on screen) into a top-down 32bpp
/// BGRA pixel buffer, returned alongside its width/height.
fn capture_hwnd_raw(hwnd: HWND) -> Result<(u32, u32, Vec<u8>), String> {
    let mut rect = Default::default();
    unsafe { GetClientRect(hwnd, &mut rect) }.map_err(|e| format!("GetClientRect: {e}"))?;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    if width <= 0 || height <= 0 {
        return Err(format!(
            "window has no visible client area ({width}x{height})"
        ));
    }

    unsafe {
        let window_dc = GetDC(Some(hwnd));
        if window_dc.is_invalid() {
            return Err("GetDC failed".into());
        }
        let mem_dc = CreateCompatibleDC(Some(window_dc));
        if mem_dc.is_invalid() {
            ReleaseDC(Some(hwnd), window_dc);
            return Err("CreateCompatibleDC failed".into());
        }

        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            // Negative height: a top-down DIB, matching the row order we want to encode in.
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            ..Default::default()
        };

        let mut bits_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
        let dib = CreateDIBSection(Some(mem_dc), &bmi, DIB_RGB_COLORS, &mut bits_ptr, None, 0)
            .map_err(|e| format!("CreateDIBSection: {e}"))?;
        if dib.is_invalid() || bits_ptr.is_null() {
            let _ = DeleteDC(mem_dc);
            ReleaseDC(Some(hwnd), window_dc);
            return Err("CreateDIBSection returned no pixel buffer".into());
        }

        let old_obj = SelectObject(mem_dc, HGDIOBJ(dib.0));
        let painted = PrintWindow(hwnd, mem_dc, PW_RENDERFULLCONTENT);

        let row_bytes = (width as usize) * 4;
        let pixels = if painted.as_bool() {
            let src =
                std::slice::from_raw_parts(bits_ptr as *const u8, row_bytes * height as usize);
            Some(src.to_vec())
        } else {
            None
        };

        SelectObject(mem_dc, old_obj);
        let _ = DeleteObject(dib.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(Some(hwnd), window_dc);

        let top_down =
            pixels.ok_or_else(|| "PrintWindow failed to render the window".to_string())?;
        Ok((width as u32, height as u32, top_down))
    }
}

/// Cap on the longer side after capture: a full-resolution window (1500+px) still produces a
/// PNG too large to hand back as an inline base64 tool result even after compression. A screenshot
/// exists to confirm a UI change rendered, not for pixel-level inspection, so downsampling first
/// is the right tradeoff.
const MAX_DIMENSION: u32 = 320;

/// Nearest-neighbor downsample `src` (top-down BGRA, `src_w`x`src_h`) so its longer side is at
/// most `MAX_DIMENSION`. Returns the source unchanged (and its original dimensions) if already
/// within that bound.
fn downsample(src_w: u32, src_h: u32, src: &[u8]) -> (u32, u32, Vec<u8>) {
    let longer = src_w.max(src_h);
    if longer <= MAX_DIMENSION {
        return (src_w, src_h, src.to_vec());
    }
    let scale = MAX_DIMENSION as f64 / longer as f64;
    let dst_w = ((src_w as f64 * scale).round() as u32).max(1);
    let dst_h = ((src_h as f64 * scale).round() as u32).max(1);

    let mut out = vec![0u8; (dst_w * dst_h * 4) as usize];
    for y in 0..dst_h {
        let src_y = ((y as f64 / scale) as u32).min(src_h - 1);
        for x in 0..dst_w {
            let src_x = ((x as f64 / scale) as u32).min(src_w - 1);
            let src_i = ((src_y * src_w + src_x) * 4) as usize;
            let dst_i = ((y * dst_w + x) * 4) as usize;
            out[dst_i..dst_i + 4].copy_from_slice(&src[src_i..src_i + 4]);
        }
    }
    (dst_w, dst_h, out)
}

/// Capture `hwnd`, downsample it, and PNG-encode it, returning the encoded bytes.
pub fn capture_hwnd_to_png(hwnd: HWND) -> Result<Vec<u8>, String> {
    let (raw_w, raw_h, bgra) = capture_hwnd_raw(hwnd)?;
    let (width, height, mut rgba) = downsample(raw_w, raw_h, &bgra);

    // GDI hands back BGRA; PNG wants RGBA. Swap in place rather than allocating a second buffer.
    for px in rgba.chunks_exact_mut(4) {
        px.swap(0, 2);
    }

    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(BufWriter::new(&mut out), width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|e| format!("png header: {e}"))?;
    writer
        .write_image_data(&rgba)
        .map_err(|e| format!("png data: {e}"))?;
    writer.finish().map_err(|e| format!("png finish: {e}"))?;
    Ok(out)
}
