"""Generate NSIS installer branding images (24-bit BMP) from the app icon + wordmark.

Header : 150x57  -> top banner on the interior pages
Sidebar: 164x314 -> graphic on the Welcome / Finish pages
"""
import os
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
ICONS = os.path.join(HERE, "..", "icons")
ASSETS = os.path.join(HERE, "..", "..", "assets")

icon = Image.open(os.path.join(ICONS, "icon.png")).convert("RGBA")
word = Image.open(os.path.join(ASSETS, "moonpool-text.png")).convert("RGBA")

# Trim the transparent margins off the wordmark so it scales tight.
wbbox = word.getbbox()
if wbbox:
    word = word.crop(wbbox)

# moon-pool brand blues
DEEP = (7, 68, 110)      # deep navy top of gradient
MID = (18, 130, 200)     # mid cyan-blue
WHITE = (255, 255, 255)


def vgrad(w, h, top, bottom):
    g = Image.new("RGB", (w, h))
    px = g.load()
    for y in range(h):
        t = y / max(1, h - 1)
        px[0, y] = (
            round(top[0] + (bottom[0] - top[0]) * t),
            round(top[1] + (bottom[1] - top[1]) * t),
            round(top[2] + (bottom[2] - top[2]) * t),
        )
    for y in range(h):
        c = px[0, y]
        for x in range(1, w):
            px[x, y] = c
    return g


def paste_fit(dst, src, box_w, box_h, cx, cy):
    r = min(box_w / src.width, box_h / src.height)
    s = src.resize((max(1, round(src.width * r)), max(1, round(src.height * r))), Image.LANCZOS)
    dst.paste(s, (round(cx - s.width / 2), round(cy - s.height / 2)), s)


# ---- Header 150x57: white bg, small icon left, wordmark right ----
hdr = Image.new("RGB", (150, 57), WHITE)
paste_fit(hdr, icon, 44, 44, 28, 28)
# recolor wordmark stays its native blue; place to the right of the icon
paste_fit(hdr, word, 92, 30, 100, 29)
hdr.save(os.path.join(HERE, "header.bmp"))

# ---- Sidebar 164x314: blue gradient, icon centered upper, wordmark below ----
side = vgrad(164, 314, DEEP, MID)
# white wordmark for contrast on the blue
wr, wg, wb, wa = word.split()
white_word = Image.merge("RGBA", (
    wr.point(lambda _: 255), wg.point(lambda _: 255), wb.point(lambda _: 255), wa,
))
paste_fit(side, icon, 104, 104, 82, 120)
paste_fit(side, white_word, 132, 34, 82, 210)
side.save(os.path.join(HERE, "sidebar.bmp"))

print("wrote header.bmp (150x57) and sidebar.bmp (164x314)")
