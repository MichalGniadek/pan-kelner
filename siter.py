import json
import requests
import os
import datetime
from imager import remap_img
from jinja2 import Environment, FileSystemLoader

with open("result.json", "r", encoding="utf-8") as f:
    data = json.load(f)

talerz_image = requests.get(data["talerz"], stream=True).raw
talerz_image.decode_content = True

img = remap_img(talerz_image)

today = datetime.datetime.today()
monday = today - datetime.timedelta(days=today.weekday())
sunday = monday + datetime.timedelta(days=6)

os.makedirs("_site", exist_ok=True)
img.save("_site/transformed.png")
with open("_site/index.html", "w", encoding="utf-8") as f:
    env = Environment(loader=FileSystemLoader("templates"))
    text = env.get_template("index.html").render(date=f"{monday.strftime('%d.%m')}-{sunday.strftime('%d.%m')}",
                                                 emalia=data["emalia"],
                                                 talerz_img=f"transformed.png?day={today.day}")
    f.write(text)
