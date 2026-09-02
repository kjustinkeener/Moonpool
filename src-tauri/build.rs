use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Bake the build date (UTC, YYYY-MM-DD) in as an env var for the About/installer
    // UI. Computed dep-free from the epoch via a civil-calendar conversion.
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=MOONPOOL_BUILD_DATE={}", ymd_utc(secs));
    // Rebuild so the date refreshes each build rather than being cached.
    println!("cargo:rerun-if-changed=build.rs");

    tauri_build::build()
}

/// Convert a Unix timestamp (secs) to a UTC "YYYY-MM-DD" string (Howard Hinnant's
/// days-from-civil, inverted).
fn ymd_utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}
