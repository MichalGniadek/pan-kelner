import json
from time import sleep
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver import Firefox


def search(text: str) -> str:
    return f"/*[text()[contains(.,'{text}')]]"


def scroll(y: int) -> str:
    return f"window.scrollBy(0, {y});"


scrollTo = "arguments[0].scrollIntoView(true);"


def clear_cookies(driver: Firefox):
    driver.get("https://www.facebook.com")

    cookie_button = driver.find_element(
        By.XPATH, f"/{search('Zezwól tylko na niezbędne pliki cookie')}")
    if cookie_button:
        cookie_button.click()
        sleep(1)


def scrape_emalia(driver: Firefox):
    driver.get("https://www.facebook.com/Emaliazablocie")

    ActionChains(driver).key_down(Keys.ESCAPE).key_up(Keys.ESCAPE).perform()
    sleep(1)

    driver.execute_script(scroll(1000))
    sleep(1)

    display_more = driver.find_element(By.XPATH,
                                       f"/{search('L U N C H  M E N U')}/../../{search('Zobacz więcej')}")

    driver.execute_script(scrollTo, display_more)
    driver.execute_script(scroll(-200))

    ActionChains(driver).move_to_element(display_more).click().perform()

    text = driver.find_element(
        By.XPATH, f"/{search('L U N C H  M E N U')}/../..").text

    for sep in ["PONIEDZIAŁEK",
                "WTOREK",
                "ŚRODA",
                "CZWARTEK",
                "PIĄTEK",
                "Rezerwujcie stoliki"]:
        text = text.replace(sep, "~")

    text = text.split("~")

    header, menu = text[0], text[1:-1]
    menu = [[opt.strip() for opt in day.strip().splitlines()] for day in menu]
    return menu


def scrape_talerz(driver: Firefox):
    driver.get("https://www.facebook.com/talerzrestaurant")

    ActionChains(driver).key_down(Keys.ESCAPE).key_up(Keys.ESCAPE).perform()
    sleep(1)

    driver.execute_script(scroll(1000))
    sleep(1)

    thumbnail = driver.find_element(
        By.XPATH, f"/{search('lunch menu')}/../../../../../../..//img[contains(@alt, 'PONIEDZIAŁEK')]")
    driver.execute_script(scrollTo, thumbnail)
    ActionChains(driver).move_to_element(thumbnail).click().perform()
    sleep(1)

    for img in driver.find_elements(
            By.XPATH, "//img[contains(@alt, 'PONIEDZIAŁEK')]"):

        if img != thumbnail and not img.get_dom_attribute("width") and not img.get_dom_attribute("height"):
            expanded_image = img
            break

    url = expanded_image.get_attribute("src")
    return url


driver = webdriver.Firefox()
driver.implicitly_wait(30)
clear_cookies(driver)
result = {
    "emalia": scrape_emalia(driver),
    "talerz": scrape_talerz(driver),
}
driver.close()

with open("result.json", 'w', encoding='utf-8') as f:
    json.dump(result, f)
