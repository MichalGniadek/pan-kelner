from PIL import Image
from PIL.ImageOps import grayscale
import numpy as np
from colour import Color


def col2arr(color):
    return [color.red * 255, color.green * 255, color.blue * 255]


remap = {"min": 140, "max": 220}
grad = [col2arr(c)for c in Color(
    "#e6ecfe").range_to(Color("#061577"), remap["max"]-remap["min"])]


def get_col(i: int):
    if i < remap["min"]:
        return col2arr(Color("#e6ecfe"))
    elif i >= remap["max"]:
        return col2arr(Color("#061577"))

    return grad[i - remap["min"]]


def remap_img(talerz_image):
    img = Image.open(talerz_image)
    img = grayscale(img)
    mapped = np.array([get_col(i) for i in range(256)],
                      dtype="uint8")[np.array(img)]
    img = Image.fromarray(mapped, "RGB")
    return img
