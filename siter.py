import json
import requests
import os
from imager import remap_img
from jinja2 import Environment, FileSystemLoader

with open("result.json", "r", encoding="utf-8") as f:
    data = json.load(f)

talerz_image = requests.get(data["talerz"], stream=True).raw
talerz_image.decode_content = True

img = remap_img(talerz_image)
img.save("_site/transformed.png")

os.makedirs("_site")
with open("_site/index.html", "w", encoding="utf-8") as f:
    env = Environment(loader=FileSystemLoader("templates"))
    text = env.get_template("index.html").render(
        emalia=data["emalia"], talerz_img="transformed.png")
    f.write(text)
