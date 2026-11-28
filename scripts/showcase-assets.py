"""Draws the pictures, tables and videos of the showcase examples in docs/authoring/examples.

Everything here is made by code, so the files belong to the project and carry the license of the
examples (CC0-1.0). Run `python3 scripts/showcase-assets.py [example ...]`; the videos need ffmpeg.
"""

import math
import os
import shutil
import subprocess
import sys
import tempfile

import numpy as np
from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXAMPLES = os.path.join(ROOT, "docs", "authoring", "examples")


def out(example, name):
    folder = os.path.join(EXAMPLES, example, "assets")
    os.makedirs(folder, exist_ok=True)
    return os.path.join(folder, name)


def save(image, example, name):
    path = out(example, name)
    if name.endswith(".jpg"):
        image.convert("RGB").save(path, quality=88, optimize=True, progressive=True)
    elif name.endswith(".webp"):
        image.save(path, quality=88, method=6)
    else:
        image.save(path, optimize=True)


def noise(w, h, seed, octaves=5, base=4, wrap=False):
    """Fractal value noise from 0 to 1, made of upscaled random grids."""
    rng = np.random.default_rng(seed)
    total = np.zeros((h, w), np.float32)
    weight, amp = 0.0, 1.0
    for o in range(octaves):
        cells = base * 2**o
        grid = rng.random((cells, cells)).astype(np.float32)
        if wrap:
            grid = np.tile(grid, (3, 3))
            big = Image.fromarray(grid).resize((w * 3, h * 3), Image.BICUBIC)
            layer = np.asarray(big)[h : 2 * h, w : 2 * w]
        else:
            layer = np.asarray(Image.fromarray(grid).resize((w, h), Image.BICUBIC))
        total += layer * amp
        weight += amp
        amp *= 0.5
    total /= weight
    return np.clip((total - total.min()) / (total.max() - total.min() + 1e-6), 0, 1)


def ramp(t, stops):
    """Maps values from 0 to 1 through a list of (position, (r, g, b)) stops."""
    t = np.clip(t, 0, 1)
    out_ = np.zeros(t.shape + (3,), np.float32)
    for (p0, c0), (p1, c1) in zip(stops, stops[1:]):
        k = np.clip((t - p0) / max(p1 - p0, 1e-6), 0, 1)[..., None]
        inside = ((t >= p0) & (t <= p1))[..., None]
        out_ = np.where(inside, np.array(c0) * (1 - k) + np.array(c1) * k, out_)
    return out_


def picture(rgb, alpha=None):
    rgb = np.clip(rgb, 0, 255).astype(np.uint8)
    if alpha is None:
        return Image.fromarray(rgb, "RGB")
    a = np.clip(alpha * 255, 0, 255).astype(np.uint8)
    return Image.fromarray(np.dstack([rgb, a]), "RGBA")


def stars(w, h, seed, count, big=0.02):
    rng = np.random.default_rng(seed)
    layer = Image.new("L", (w, h), 0)
    pen = ImageDraw.Draw(layer)
    for _ in range(count):
        x, y = rng.random() * w, rng.random() * h
        r = 2.2 if rng.random() < big else rng.choice([0.6, 0.9, 1.2])
        pen.ellipse((x - r, y - r, x + r, y + r), fill=int(120 + rng.random() * 135))
    glow = layer.filter(ImageFilter.GaussianBlur(3))
    return np.maximum(np.asarray(layer, np.float32), np.asarray(glow, np.float32) * 2.5) / 255


def font(size):
    for path in ("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",):
        if os.path.exists(path):
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def grid(w, h):
    y, x = np.mgrid[0:h, 0:w].astype(np.float32)
    return x, y


# effects-distort: a portal in a nebula


def nebula(w=1920, h=1080, seed=11):
    n = noise(w, h, seed, 7, 3)
    m = noise(w, h, seed + 1, 6, 2)
    x, y = grid(w, h)
    band = np.exp(-(((y / h - 0.5) - 0.25 * np.sin(x / w * 3.2)) ** 2) / 0.08)
    cloud = np.clip(n * 1.4 * (0.35 + band) - 0.35, 0, 1)
    colors = ramp(m, [(0, (40, 10, 90)), (0.45, (150, 40, 160)), (0.7, (20, 120, 190)), (1, (60, 220, 210))])
    base = ramp(y / h, [(0, (6, 4, 22)), (1, (14, 6, 36))])
    rgb = base + colors * cloud[..., None] * 1.7
    rgb += (stars(w, h, seed, 700) * 230)[..., None]
    return picture(rgb)


def portal(size=900, seed=5):
    x, y = grid(size, size)
    cx = (x - size / 2) / (size / 2)
    cy = (y - size / 2) / (size / 2)
    r = np.sqrt(cx**2 + cy**2)
    a = np.arctan2(cy, cx)
    n = noise(size, size, seed, 5, 4)
    arms = 0.5 + 0.5 * np.cos(3 * a + 7 * np.log(r + 0.05) + n * 2.5)
    body = np.clip(1 - r, 0, 1) ** 1.2
    light = arms**3 * body * 1.4 + np.exp(-r * 9) * 1.6
    rgb = ramp(np.clip(light, 0, 1), [(0, (30, 0, 80)), (0.4, (200, 50, 220)), (0.75, (120, 230, 255)), (1, (255, 255, 255))])
    alpha = np.clip(light * 1.6, 0, 1) * np.clip((1 - r) * 4, 0, 1)
    return picture(rgb, alpha)


def ripples(size=512, seed=3):
    nx = noise(size, size, seed, 4, 4, wrap=True)
    ny = noise(size, size, seed + 9, 4, 4, wrap=True)
    rgb = np.dstack([nx * 255, ny * 255, np.full_like(nx, 128)])
    return picture(rgb)


def vines(w=1920, h=520, seed=21):
    rng = np.random.default_rng(seed)
    layer = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    glow = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    pen, halo = ImageDraw.Draw(layer), ImageDraw.Draw(glow)
    for i in range(26):
        x0 = (i + rng.random() * 0.6) * w / 26
        length = h * (0.35 + rng.random() * 0.6)
        points = [(x0 + 18 * math.sin(t / 9 + i), t) for t in np.linspace(0, length, 40)]
        pen.line(points, fill=(30, 18, 60, 255), width=5)
        for k in range(3, 40, 5):
            px, py = points[k]
            side = 1 if k % 10 else -1
            pen.polygon([(px, py), (px + side * 22, py + 8), (px + side * 6, py + 20)], fill=(48, 26, 90, 255))
        tip = points[-1]
        hue = [(120, 240, 255), (240, 120, 255), (160, 255, 200)][i % 3]
        s = 9 + rng.random() * 8
        pen.polygon([(tip[0], tip[1] - s), (tip[0] + s * 0.6, tip[1]), (tip[0], tip[1] + s * 2), (tip[0] - s * 0.6, tip[1])], fill=hue + (255,))
        halo.ellipse((tip[0] - 40, tip[1] - 30, tip[0] + 40, tip[1] + 50), fill=hue + (150,))
    glow = glow.filter(ImageFilter.GaussianBlur(18))
    return Image.alpha_composite(glow, layer)


def card(w=520, h=760):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    pen.rounded_rectangle((0, 0, w - 1, h - 1), 36, fill=(22, 12, 48, 235), outline=(255, 210, 120, 255), width=6)
    pen.rounded_rectangle((22, 22, w - 23, h - 23), 26, outline=(255, 210, 120, 160), width=2)
    cx, cy = w / 2, h / 2 - 30
    for k, radius in enumerate((190, 150, 96)):
        pen.ellipse((cx - radius, cy - radius, cx + radius, cy + radius), outline=(255, 214, 140, 230 - k * 40), width=4)
    for k in range(12):
        a = k * math.pi / 6
        pen.line((cx + 96 * math.cos(a), cy + 96 * math.sin(a), cx + 190 * math.cos(a), cy + 190 * math.sin(a)), fill=(255, 214, 140, 200), width=3)
    star = [(cx + (150 if k % 2 == 0 else 60) * math.cos(k * math.pi / 5 - math.pi / 2), cy + (150 if k % 2 == 0 else 60) * math.sin(k * math.pi / 5 - math.pi / 2)) for k in range(10)]
    pen.polygon(star, fill=(255, 196, 90, 255))
    pen.ellipse((cx - 30, cy - 30, cx + 30, cy + 30), fill=(120, 230, 255, 255))
    text = "THE PORTAL"
    pen.text((cx, h - 110), text, font=font(48), fill=(255, 220, 150, 255), anchor="mm")
    glow = image.filter(ImageFilter.GaussianBlur(10))
    return Image.alpha_composite(glow, image)


def distort():
    name = "effects-distort"
    save(nebula(), name, "nebula.jpg")
    save(portal(), name, "portal.webp")
    save(ripples(), name, "ripples.png")
    save(vines(), name, "vines.webp")
    save(card(), name, "card.png")


# effects-reveal: a valley from day to night


def ridges(w, h, seed):
    """Heights of five mountain ridges from the back to the front, as a list of arrays of x."""
    out = []
    for k in range(5):
        line = noise(w, 1, seed + k, 6, 3)[0] if False else noise(w, 8, seed + k, 6, 3)[4]
        base = h * (0.42 + k * 0.11)
        out.append(base - line * h * (0.28 - k * 0.035))
    return out


def valley(w, h, seed, palette):
    x, y = grid(w, h)
    sky = ramp(y / h, palette["sky"])
    rgb = sky.copy()
    if palette.get("stars"):
        rgb += (stars(w, h, seed + 40, 900) * 210)[..., None] * (1 - y / h)[..., None]
    sx, sy, sr = palette["sun"]
    d = np.sqrt((x - sx) ** 2 + (y - sy) ** 2)
    rgb += np.array(palette["glow"]) * np.exp(-d / (sr * 3))[..., None]
    rgb = np.where((d < sr)[..., None], np.array(palette["disc"]), rgb)
    haze = noise(w, h, seed + 7, 5, 3)
    for k, top in enumerate(ridges(w, h, seed)):
        inside = y >= top[None, :]
        shade = np.array(palette["hills"][k], np.float32)
        tone = shade * (0.85 + 0.3 * haze[..., None]) * (1 - 0.25 * ((y - top[None, :]) / h)[..., None])
        rgb = np.where(inside[..., None], tone, rgb)
        mist = np.exp(-np.maximum(y - top[None, :], 0) / 40) * inside * 0.35
        rgb = rgb * (1 - mist[..., None]) + np.array(palette["mist"]) * mist[..., None]
    return picture(rgb)


DAY = {
    "sky": [(0, (90, 160, 230)), (0.55, (250, 205, 170)), (1, (255, 230, 200))],
    "sun": (1380, 330, 70), "glow": (120, 90, 40), "disc": (255, 246, 220),
    "hills": [(150, 160, 200), (110, 140, 170), (80, 125, 120), (50, 100, 80), (30, 70, 55)],
    "mist": (240, 230, 225),
}
NIGHT = {
    "sky": [(0, (6, 8, 30)), (0.6, (30, 30, 80)), (1, (70, 50, 110))], "stars": True,
    "sun": (1560, 190, 55), "glow": (60, 70, 120), "disc": (240, 240, 255),
    "hills": [(40, 40, 90), (30, 32, 75), (22, 26, 60), (14, 20, 45), (8, 12, 30)],
    "mist": (90, 90, 150),
}


def sweep(w=960, h=540, seed=8):
    x, y = grid(w, h)
    t = (x / w * 0.75 + (1 - y / h) * 0.25) * 0.8 + noise(w, h, seed, 5, 3) * 0.2
    v = (t - t.min()) / (t.max() - t.min()) * 255
    return picture(np.dstack([v, v, v]))


def puffs(size=512, seed=2):
    n = noise(size, size, seed, 6, 3, wrap=True)
    v = np.clip((n - 0.35) * 2.2, 0, 1) * 255
    return picture(np.dstack([v, v, v]))


def mist(w=1920, h=600, seed=31):
    x, y = grid(w, h)
    n = noise(w, h, seed, 6, 3)
    fade = np.sin(np.clip(y / h, 0, 1) * math.pi) ** 2
    a = np.clip((n - 0.5) * 2.5, 0, 1) * fade
    return picture(np.full((h, w, 3), 235.0), a)


def owl_tree(w=760, h=1080):
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    ink = (10, 8, 20, 255)
    pen.polygon([(120, h), (230, 0), (330, 0), (300, h)], fill=ink)
    for (x0, y0, x1, y1, width) in ((260, 420, 700, 360, 34), (250, 650, 40, 560, 26), (280, 200, 560, 120, 18), (520, 380, 620, 300, 12)):
        pen.line((x0, y0, x1, y1), fill=ink, width=width)
    rng = np.random.default_rng(4)
    for cx0, cy0, spread in ((640, 330, 60), (110, 540, 55), (540, 100, 50), (620, 280, 30)):
        for _ in range(480):
            cx, cy = cx0 + rng.normal(0, spread * 1.3), cy0 + rng.normal(0, spread * 0.6)
            a, r = rng.uniform(0, math.pi), rng.uniform(10, 22)
            dx, dy = r * math.cos(a), r * 0.35 * math.sin(a)
            pen.polygon([(cx - dx, cy - dy), (cx + dy, cy - dx * 0.25), (cx + dx, cy + dy), (cx - dy, cy + dx * 0.25)], fill=ink)
    ox, oy = 470, 290
    pen.ellipse((ox - 70, oy - 60, ox + 70, oy + 110), fill=(26, 20, 36, 255))
    pen.ellipse((ox - 62, oy - 90, ox + 62, oy + 10), fill=(26, 20, 36, 255))
    pen.polygon([(ox - 60, oy - 70), (ox - 52, oy - 120), (ox - 25, oy - 82)], fill=(26, 20, 36, 255))
    pen.polygon([(ox + 60, oy - 70), (ox + 52, oy - 120), (ox + 25, oy - 82)], fill=(26, 20, 36, 255))
    pen.polygon([(ox - 8, oy - 30), (ox + 8, oy - 30), (ox, oy - 12)], fill=(230, 170, 60, 255))
    return image


def iris(size=128):
    x, y = grid(size, size)
    cx, cy = (x - size / 2) / (size / 2), (y - size / 2) / (size / 2)
    r = np.sqrt(cx**2 + cy**2)
    a = np.arctan2(cy, cx)
    streaks = 0.75 + 0.25 * np.cos(a * 23) * np.cos(a * 7)
    rgb = ramp(r, [(0, (255, 220, 90)), (0.5, (250, 160, 30)), (0.62, (140, 60, 10)), (1, (250, 240, 225))])
    rgb *= np.where(r < 0.62, streaks, 1)[..., None]
    rgb = np.where((r < 0.24)[..., None], np.array((8, 6, 10)), rgb)
    rgb = np.where(((cx + 0.2) ** 2 + (cy + 0.2) ** 2 < 0.012)[..., None], np.array((255, 255, 255)), rgb)
    return picture(rgb)


def reveal():
    name = "effects-reveal"
    save(valley(1920, 1080, 14, DAY), name, "day.jpg")
    save(valley(1920, 1080, 14, NIGHT), name, "night.jpg")
    save(sweep(), name, "sweep.png")
    save(puffs(), name, "puffs.png")
    save(mist(), name, "mist.png")
    save(owl_tree(), name, "tree.png")
    save(iris(), name, "iris.png")


# effects-color: a harbour at sunset in five grades, and a balloon filmed on a green screen


def harbour(w=1920, h=1080, seed=17):
    rng = np.random.default_rng(seed)
    x, y = grid(w, h)
    horizon = int(h * 0.62)
    sky = ramp(y / horizon, [(0, (40, 30, 90)), (0.45, (200, 80, 110)), (0.8, (255, 160, 80)), (1, (255, 220, 140))])
    sun = np.exp(-np.sqrt((x - w * 0.5) ** 2 + (y - horizon + 60) ** 2) / 90)
    rgb = sky + np.array((255, 200, 120)) * sun[..., None] * 0.9
    rgb = np.where((np.sqrt((x - w * 0.5) ** 2 + (y - horizon + 60) ** 2) < 70)[..., None], np.array((255, 236, 190)), rgb)
    clouds = noise(w, h, seed, 6, 3)
    streak = np.exp(-((y / h - 0.22 - 0.05 * np.sin(x / w * 5)) ** 2) / 0.004)
    rgb += np.array((255, 140, 160)) * (np.clip(clouds - 0.45, 0, 1) * 2.2 * streak)[..., None]
    image = picture(rgb)
    pen = ImageDraw.Draw(image)
    left = 0
    while left < w:
        bw = int(rng.integers(50, 140))
        bh = int(rng.integers(80, 360)) if abs(left + bw / 2 - w / 2) > 260 else int(rng.integers(40, 120))
        shade = tuple(int(v) for v in rng.integers(18, 46, 3))
        pen.rectangle((left, horizon - bh, left + bw - 4, horizon), fill=shade)
        for wy in range(horizon - bh + 12, horizon - 8, 18):
            for wx in range(left + 8, left + bw - 12, 16):
                if rng.random() < 0.35:
                    pen.rectangle((wx, wy, wx + 7, wy + 9), fill=(255, 210 + int(rng.integers(0, 40)), 120))
        if rng.random() < 0.3:
            pen.line((left + bw // 2, horizon - bh, left + bw // 2, horizon - bh - 60), fill=shade, width=4)
        left += bw
    sky_part = np.asarray(image, np.float32)[:horizon]
    water = sky_part[::-1][: h - horizon]
    ripple = noise(w, h - horizon, seed + 3, 5, 6)
    yy = np.arange(h - horizon, dtype=np.float32)[:, None]
    shift = np.round(np.sin(yy / 5 + ripple * 6) * (1 + yy / 60)).astype(int)
    cols = np.clip(np.arange(w)[None, :] + shift, 0, w - 1)
    water = water[np.arange(h - horizon)[:, None], cols] * (0.55 + 0.1 * ripple[..., None])
    water += np.array((10, 20, 50))
    water = np.asarray(picture(water).filter(ImageFilter.GaussianBlur(1.2)), np.float32)
    rgb = np.concatenate([sky_part, water], 0)
    return picture(rgb)


def teal_orange(size=17):
    lines = ["TITLE \"Teal and orange\"", f"LUT_3D_SIZE {size}", "DOMAIN_MIN 0 0 0", "DOMAIN_MAX 1 1 1"]
    for b in range(size):
        for g in range(size):
            for r in range(size):
                c = np.array([r, g, b], np.float32) / (size - 1)
                luma = c @ np.array([0.299, 0.587, 0.114])
                warm, cool = np.array([1.0, 0.62, 0.32]), np.array([0.1, 0.55, 0.62])
                graded = luma * (cool + (warm - cool) * np.clip(luma * 1.4 - 0.2, 0, 1)) * 1.25
                curve = graded * graded * (3 - 2 * graded)
                out_ = np.clip(c * 0.35 + curve * 0.65, 0, 1)
                lines.append(" ".join(f"{v:.5f}" for v in (out_[0], out_[1], out_[2])))
    return "\n".join(lines) + "\n"


def balloon_frame(t, size=480):
    x, y = grid(size, size)
    cx, cy = size / 2 + 24 * math.sin(t * 2 * math.pi), size * 0.36 + 10 * math.sin(t * 4 * math.pi)
    dx, dy = (x - cx) / 150, (y - cy) / 150
    half = np.where(dy < 0, np.sqrt(np.clip(1 - dy**2, 0, 1)), np.clip(1 - dy * 0.75, 0, 1) ** 0.8 * np.sqrt(np.clip(1 - (dy / 1.25) ** 2, 0, 1)))
    inside = (np.abs(dx) < half) & (dy > -1) & (dy < 1.2)
    u = np.arcsin(np.clip(dx / np.maximum(half, 1e-3), -1, 1)) + t * 2 * math.pi
    stripes = np.array([(240, 60, 70), (255, 190, 40), (60, 140, 240), (250, 250, 245), (150, 70, 200), (40, 200, 190)])
    index = np.floor(u / (math.pi / 6)).astype(int) % len(stripes)
    light = 0.55 + 0.45 * np.cos(np.arcsin(np.clip(dx / np.maximum(half, 1e-3), -1, 1)) + 0.5) * (1 - 0.25 * dy)
    green = np.dstack([x * 0 + 24, 190 + 25 * np.cos(x / size * 3) * np.cos(y / size * 2), x * 0 + 48])
    rgb = np.where(inside[..., None], stripes[index] * light[..., None], green)
    image = picture(rgb)
    pen = ImageDraw.Draw(image)
    bottom = cy + 150 * 1.2
    for side in (-1, 1):
        pen.line((cx + side * 34, bottom - 6, cx + side * 22, bottom + 52), fill=(70, 45, 30), width=3)
    pen.rectangle((cx - 26, bottom + 52, cx + 26, bottom + 90), fill=(160, 100, 52), outline=(96, 58, 26), width=3)
    pen.line((cx - 26, bottom + 66, cx + 26, bottom + 66), fill=(96, 58, 26), width=2)
    return image


def greenscreen(path, frames=48, fps=24):
    with tempfile.TemporaryDirectory() as work:
        for i in range(frames):
            balloon_frame(i / frames).save(os.path.join(work, f"f{i:03d}.png"))
        subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-framerate", str(fps), "-i", os.path.join(work, "f%03d.png"),
                        "-c:v", "libvpx-vp9", "-b:v", "0", "-crf", "34", "-pix_fmt", "yuv420p", "-an", path], check=True)


def light_leak(w=1920, h=480):
    x, y = grid(w, h)
    glow = np.exp(-((x / w - 0.25) ** 2) / 0.05 - (y / h) ** 2 / 0.25) + 0.7 * np.exp(-((x / w - 0.8) ** 2) / 0.02 - (y / h) ** 2 / 0.12)
    rgb = ramp(x / w, [(0, (255, 120, 60)), (0.5, (255, 60, 140)), (1, (90, 120, 255))])
    return picture(rgb, np.clip(glow, 0, 1) * 0.9)


def color():
    name = "effects-color"
    save(harbour(), name, "harbour.jpg")
    with open(out(name, "teal-orange.cube"), "w") as f:
        f.write(teal_orange())
    greenscreen(out(name, "balloon.webm"))
    save(light_leak(), name, "leak.png")


# effects-light: a relic that shines, sparkles, pulses and crackles


def velvet(w=1920, h=1080, seed=41):
    x, y = grid(w, h)
    r = np.sqrt(((x - w / 2) / w) ** 2 + ((y - h / 2) / h) ** 2)
    rgb = ramp(r * 1.6, [(0, (46, 22, 70)), (0.5, (20, 10, 40)), (1, (4, 3, 10))])
    a = np.arctan2(y - h / 2, x - w / 2)
    rays = (0.5 + 0.5 * np.cos(a * 24)) ** 8 * np.exp(-r * 3) * 0.25
    rgb += np.array((255, 200, 120)) * rays[..., None] * 0.5
    rgb *= (0.92 + 0.08 * noise(w, h, seed, 4, 16))[..., None]
    return picture(rgb)


def medallion(size=640):
    x, y = grid(size, size)
    cx, cy = (x - size / 2) / (size / 2), (y - size / 2) / (size / 2)
    r = np.sqrt(cx**2 + cy**2)
    a = np.arctan2(cy, cx)
    rim = ((r > 0.78) & (r < 0.98)) | ((r > 0.52) & (r < 0.6))
    teeth = (r >= 0.6) & (r <= 0.78) & (np.cos(a * 16) > 0.2)
    metal = 0.6 + 0.4 * np.cos(a - 0.8) * np.where(r > 0.78, 1, -1)
    gold = ramp(np.clip(metal, 0, 1), [(0, (110, 60, 10)), (0.5, (220, 160, 50)), (1, (255, 240, 170))])
    gem_r = 0.42
    gem = r < gem_r
    facet = 0.5 + 0.5 * np.cos(np.floor((a + math.pi) / (math.pi / 4)) * 1.7 + r * 6)
    ruby = ramp(facet * (1 - r / gem_r * 0.4), [(0, (70, 0, 20)), (0.6, (210, 20, 60)), (1, (255, 150, 170))])
    rgb = np.where(gem[..., None], ruby, gold)
    alpha = (rim | teeth | gem).astype(np.float32)
    image = picture(rgb, alpha)
    glare = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    ImageDraw.Draw(glare).ellipse((size * 0.38, size * 0.32, size * 0.46, size * 0.38), fill=(255, 255, 255, 220))
    return Image.alpha_composite(image, glare.filter(ImageFilter.GaussianBlur(4)))


def orb(size=300, seed=6):
    x, y = grid(size, size)
    cx, cy = (x - size / 2) / (size / 2), (y - size / 2) / (size / 2)
    r = np.sqrt(cx**2 + cy**2)
    n = noise(size, size, seed, 5, 4)
    v = np.clip(1 - r, 0, 1) ** 0.7 * (0.4 + 0.6 * n)
    rgb = ramp(v, [(0, (10, 20, 60)), (0.5, (60, 120, 230)), (1, (230, 245, 255))])
    return picture(rgb, np.clip((1 - r) * 8, 0, 1))


def runes(size=1000):
    image = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    c = size / 2
    pen.ellipse((40, 40, size - 40, size - 40), outline=(255, 214, 150, 255), width=3)
    pen.ellipse((110, 110, size - 110, size - 110), outline=(255, 214, 150, 200), width=2)
    rng = np.random.default_rng(12)
    for k in range(24):
        a = k / 24 * 2 * math.pi
        x, y = c + 410 * math.cos(a), c + 410 * math.sin(a)
        mark = Image.new("RGBA", (80, 80), (0, 0, 0, 0))
        stroke = ImageDraw.Draw(mark)
        stroke.line((40, 14, 40, 66), fill=(255, 220, 160, 255), width=4)
        for _ in range(int(rng.integers(1, 4))):
            y0 = float(rng.choice([16, 28, 40, 52]))
            side = float(rng.choice([-1, 1]))
            stroke.line((40, y0, 40 + side * 18, y0 + float(rng.choice([-12, 0, 12]))), fill=(255, 220, 160, 255), width=4)
        mark = mark.rotate(-math.degrees(a) - 90, resample=Image.BICUBIC)
        image.alpha_composite(mark, (int(x - 40), int(y - 40)))
    return image


def comets(size=900):
    image = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    for (x, y), color in (((size - 40, size / 2), (255, 214, 130)), ((40, size / 2), (150, 230, 255))):
        for k, (r, a) in enumerate(((26, 90), (16, 200), (9, 255))):
            pen.ellipse((x - r, y - r, x + r, y + r), fill=color + (a,))
    return image.filter(ImageFilter.GaussianBlur(1.5))


def light():
    name = "effects-light"
    save(comets(), name, "comets.png")
    save(velvet(), name, "velvet.jpg")
    save(medallion(), name, "medallion.png")
    save(orb(), name, "orb.png")
    save(runes(), name, "runes.png")


# blend-modes: one swatch over a backdrop in each of the blend modes


def backdrop(w=1920, h=1080, seed=51):
    x, y = grid(w, h)
    t = x / w * 0.7 + y / h * 0.3
    rgb = ramp(t, [(0, (30, 60, 140)), (0.35, (40, 150, 160)), (0.65, (240, 170, 70)), (1, (200, 50, 90))])
    rng = np.random.default_rng(seed)
    layer = np.zeros((h, w), np.float32)
    for _ in range(70):
        cx, cy, r = rng.random() * w, rng.random() * h, rng.uniform(20, 90)
        layer += np.exp(-((x - cx) ** 2 + (y - cy) ** 2) / (2 * r * r)) * rng.uniform(0.2, 0.6)
    rgb = rgb * (0.75 + 0.25 * noise(w, h, seed, 4, 3))[..., None] + 255 * np.clip(layer, 0, 1)[..., None] * 0.45
    stripes = (np.sin((x + y) / 9) > 0.6) * 0.08
    return picture(rgb * (1 - stripes[..., None]))


def swatch(size=300):
    x, y = grid(size, size)
    cx, cy = (x - size / 2) / (size / 2), (y - size / 2) / (size / 2)
    a = (np.arctan2(cy, cx) / (2 * math.pi) + 0.5) * 6
    k = np.floor(a) % 6
    f = a - np.floor(a)
    hues = np.array([(255, 40, 40), (255, 220, 30), (40, 220, 70), (30, 220, 230), (50, 70, 255), (230, 40, 230)], np.float32)
    rgb = hues[k.astype(int)] * (1 - f[..., None]) + hues[((k + 1) % 6).astype(int)] * f[..., None]
    r = np.sqrt(cx**2 + cy**2)
    rgb = rgb * np.clip(r * 1.3, 0, 1)[..., None] + 255 * np.clip(1 - r * 1.3, 0, 1)[..., None]
    rgb *= np.where(r > 0.62, 0.35 + 0.65 * (cy + 1) / 2, 1)[..., None]
    alpha = np.clip((0.95 - r) * 30, 0, 1)
    return picture(rgb, alpha)


def blends():
    name = "blend-modes"
    save(backdrop(), name, "backdrop.jpg")
    save(swatch(), name, "swatch.png")


# timelines: a fair at night with fireworks that start on events


def fair(w=1920, h=1080, seed=61):
    rng = np.random.default_rng(seed)
    x, y = grid(w, h)
    rgb = ramp(y / h, [(0, (8, 6, 30)), (0.6, (40, 20, 70)), (0.85, (110, 40, 80)), (1, (30, 12, 30))])
    rgb += (stars(w, h, seed, 500) * 200)[..., None] * (1 - y / h)[..., None] ** 2
    image = picture(rgb)
    pen = ImageDraw.Draw(image)
    left, ground = 0, int(h * 0.86)
    while left < w:
        bw = int(rng.integers(60, 160))
        bh = int(rng.integers(60, 240))
        pen.rectangle((left, ground - bh, left + bw - 6, ground), fill=(14, 10, 26))
        for wy in range(ground - bh + 14, ground - 10, 22):
            for wx in range(left + 10, left + bw - 16, 18):
                if rng.random() < 0.25:
                    pen.rectangle((wx, wy, wx + 8, wy + 10), fill=(255, 190, 110))
        left += bw
    pen.rectangle((0, ground, w, h), fill=(10, 6, 18))
    return image


def burst(size=600, seed=0, colors=((255, 210, 120), (255, 90, 140))):
    rng = np.random.default_rng(seed)
    image = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    c = size / 2
    for k in range(90):
        a = rng.random() * 2 * math.pi
        r1 = size * 0.48 * rng.uniform(0.7, 1.0)
        color = colors[k % len(colors)]
        for j in range(12):
            f = j / 12
            rr = r1 * (0.25 + 0.75 * f)
            px, py = c + rr * math.cos(a), c + rr * math.sin(a) + f * f * 18
            dot = 1.2 + 3.2 * f
            pen.ellipse((px - dot, py - dot, px + dot, py + dot), fill=color + (int(60 + 195 * f),))
    glow = image.filter(ImageFilter.GaussianBlur(6))
    return Image.alpha_composite(glow, image)


def wheel(size=720):
    image = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    pen = ImageDraw.Draw(image)
    c = size / 2
    for radius, width in ((size * 0.46, 8), (size * 0.40, 4)):
        pen.ellipse((c - radius, c - radius, c + radius, c + radius), outline=(255, 120, 190, 255), width=width)
    for k in range(16):
        a = k / 16 * 2 * math.pi
        pen.line((c, c, c + size * 0.46 * math.cos(a), c + size * 0.46 * math.sin(a)), fill=(180, 150, 255, 255), width=3)
        bx, by = c + size * 0.46 * math.cos(a), c + size * 0.46 * math.sin(a)
        pen.ellipse((bx - 7, by - 7, bx + 7, by + 7), fill=(255, 230, 150, 255))
    pen.ellipse((c - 26, c - 26, c + 26, c + 26), fill=(255, 200, 120, 255))
    glow = image.filter(ImageFilter.GaussianBlur(8))
    return Image.alpha_composite(glow, image)


def timelines():
    name = "timelines"
    save(fair(), name, "fair.jpg")
    save(burst(seed=1), name, "burst-gold.webp")
    save(burst(seed=2, colors=((120, 220, 255), (190, 120, 255))), name, "burst-blue.webp")
    save(burst(seed=3, colors=((140, 255, 160), (255, 250, 200))), name, "burst-green.webp")
    save(wheel(), name, "wheel.png")


# values-extras: a dashboard of the time, the sound, the pointer and the song


def dashboard(w=1920, h=1080, seed=71):
    x, y = grid(w, h)
    r = np.sqrt(((x - w / 2) / w) ** 2 + ((y - h / 2) / h) ** 2)
    rgb = ramp(r * 1.5, [(0, (16, 22, 44)), (0.6, (8, 10, 24)), (1, (2, 3, 8))])
    hexes = (np.abs(np.sin(x / 26) * np.sin(y / 26 + x / 52)) > 0.97) * 0.5
    lines = ((x % 120 < 1.5) | (y % 120 < 1.5)) * 0.35
    rgb += np.array((40, 120, 200)) * (hexes + lines)[..., None] * (1 - r)[..., None] * 0.6
    return picture(rgb)


def sleeve(size=512, seed=72):
    x, y = grid(size, size)
    t = x / size * 0.6 + y / size * 0.4
    rgb = ramp(t, [(0, (40, 20, 90)), (0.5, (220, 60, 140)), (1, (255, 180, 90))])
    c = size / 2
    d = np.sqrt((x - c * 1.15) ** 2 + (y - c * 1.05) ** 2) / c
    grooves = 0.75 + 0.25 * np.sin(d * 90)
    disc = d < 0.72
    vinyl = np.array((18, 16, 24)) * grooves[..., None]
    rgb = np.where(disc[..., None], vinyl, rgb)
    rgb = np.where((d < 0.22)[..., None], np.array((255, 214, 120)), rgb)
    rgb = np.where((d < 0.03)[..., None], np.array((10, 10, 10)), rgb)
    return picture(rgb)


def bulge(size=256):
    x, y = grid(size, size)
    cx, cy = (x - size / 2) / (size / 2), (y - size / 2) / (size / 2)
    r2 = cx**2 + cy**2
    h = np.clip(1 - r2, 0, 1) ** 2
    nx, ny = -cx * h * 2, -cy * h * 2
    nz = np.sqrt(np.clip(1 - nx**2 - ny**2, 0, 1))
    rgb = np.dstack([(nx * 0.5 + 0.5) * 255, (ny * 0.5 + 0.5) * 255, nz * 255])
    return picture(rgb, np.clip((1 - r2) * 4, 0, 1))


def values_extras():
    name = "values-extras"
    save(bulge(), name, "bulge.png")
    save(dashboard(), name, "dashboard.jpg")
    save(sleeve(), name, "sleeve.png")


MAKERS = {"values-extras": values_extras, "timelines": timelines, "blend-modes": blends, "effects-light": light, "effects-distort": distort, "effects-reveal": reveal, "effects-color": color}


def main():
    for name in sys.argv[1:] or MAKERS:
        MAKERS[name]()


main()
