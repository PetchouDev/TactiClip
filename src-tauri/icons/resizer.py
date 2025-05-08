from PIL import Image
import os

os.chdir(os.path.dirname(os.path.abspath(__file__)))

# Image source
src_image = "TactiClip_icon_main.png"

# Format : (filename, size)
targets = [
    ("128x128.png", 128),
    ("128x128@2x.png", 256),
    ("32x32.png", 32),
    ("Square107x107Logo.png", 107),
    ("Square142x142Logo.png", 142),
    ("Square150x150Logo.png", 150),
    ("Square284x284Logo.png", 284),
    ("Square30x30Logo.png", 30),
    ("Square310x310Logo.png", 310),
    ("Square44x44Logo.png", 44),
    ("Square71x71Logo.png", 71),
    ("Square89x89Logo.png", 89),
    ("StoreLogo.png", 50),  # Valeur standard
    ("icon.png", 256)
]

img = Image.open(src_image)

# PNG resizing
for name, size in targets:
    resized = img.resize((size, size), Image.LANCZOS)
    resized.save(name)
    print(f"Saved {name} with size {size}x{size}")


# Conversion to ICO
img.save("icon.ico", format="ICO", sizes=[(32, 32), (64, 64), (128, 128), (256, 256)])
print("Saved icon.ico with sizes 128x128 and 256x256")

# Conversion to ICNS (macOS)
img.save("icon.icns", format="ICNS")
print("Saved icon.icns")


print("All done.")
