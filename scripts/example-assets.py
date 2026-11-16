import math
import os
import random
import struct
import wave

from PIL import Image, ImageChops, ImageDraw, ImageFilter

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WIDTH, HEIGHT = 1920, 1080


def tone(path):
    rate, seconds = 8000, 0.5
    frames = int(rate * seconds)
    out = []
    for i in range(frames):
        t = i / rate
        wave_ = 0.6 * math.sin(2 * math.pi * 220 * t) + 0.3 * math.sin(2 * math.pi * 330 * t)
        out.append(int(9000 * wave_))
    with wave.open(path, "wb") as sink:
        sink.setnchannels(1)
        sink.setsampwidth(2)
        sink.setframerate(rate)
        sink.writeframes(struct.pack(f"<{frames}h", *out))


def sky():
    top, bottom = (20, 36, 66), (42, 74, 116)
    column = Image.new("RGB", (1, HEIGHT))
    for y in range(HEIGHT):
        k = y / HEIGHT
        column.putpixel((0, y), tuple(int(a + (b - a) * k) for a, b in zip(top, bottom)))
    image = column.resize((WIDTH, HEIGHT))
    rng = random.Random(7)
    clouds = Image.new("L", (WIDTH // 8, HEIGHT // 8))
    clouds.putdata([rng.randint(0, 255) for _ in range(WIDTH * HEIGHT // 64)])
    clouds = clouds.resize((WIDTH, HEIGHT), Image.BICUBIC).filter(ImageFilter.GaussianBlur(60))
    clouds = clouds.point(lambda v: max(0, (v - 110) * 2) // 3)
    haze = Image.new("RGB", (WIDTH, HEIGHT), (150, 165, 190))
    return Image.composite(haze, image, clouds.point(lambda v: v // 2)), rng


def draw():
    image, rng = sky()
    pen = ImageDraw.Draw(image)
    for _ in range(420):
        x, y, r = rng.randrange(WIDTH), rng.randrange(HEIGHT * 3 // 4), rng.choice((1, 1, 2, 2, 3))
        pen.ellipse((x - r, y - r, x + r, y + r), fill=(235, 240, 250))
    glow = Image.new("RGB", (WIDTH, HEIGHT), (0, 0, 0))
    ImageDraw.Draw(glow).ellipse((1380, 120, 1580, 320), fill=(120, 140, 180))
    image = ImageChops.add(image, glow.filter(ImageFilter.GaussianBlur(40)))
    pen = ImageDraw.Draw(image)
    pen.ellipse((1415, 155, 1545, 285), fill=(252, 253, 255))
    pen.ellipse((-400, 730, 1700, 1500), fill=(190, 204, 224))
    pen.ellipse((60, 850, 2600, 1700), fill=(228, 234, 244))
    for x, base, h in ((300, 820, 120), (420, 850, 85), (1320, 850, 140), (1450, 850, 75)):
        pen.polygon([(x - 25, base), (x + 25, base), (x, base - h)], fill=(26, 42, 70))
    grain = Image.effect_noise((WIDTH, HEIGHT), 14).convert("RGB")
    return ImageChops.blend(image, ImageChops.overlay(image, grain), 0.35)


def main():
    for scene in ("animacion-e-interaccion", "sonido"):
        tone(os.path.join(ROOT, "docs", "crear", "ejemplos", scene, "assets", "zumbido.wav"))
    draw().save(os.path.join(ROOT, "apps", "reiwand-app", "look", "pictures", "example.jpg"), quality=85, optimize=True)


main()
