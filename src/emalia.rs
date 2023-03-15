use crate::common::search;
use anyhow::Context;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

pub async fn run_emalia(driver: &WebDriver) -> anyhow::Result<Vec<Vec<String>>> {
    let text = download_text(driver).await?;
    let menu = parse_menu(&text)?;

    Ok(menu)
}

fn parse_menu(text: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let mut text = text;

    let result: anyhow::Result<Vec<Vec<String>>> = [
        "PONIEDZIAŁEK",
        "WTOREK",
        "ŚRODA",
        "CZWARTEK",
        "PIĄTEK",
        "Rezerwujcie stoliki",
    ]
    .into_iter()
    .try_fold(vec![], |mut acc, pat| {
        let i = text.find(pat).context("No weekday in text")?;
        let part = text[..i]
            .trim()
            .split("\n")
            .map(|m| m.trim().to_string())
            .collect();
        text = &text[(i + pat.len())..];

        acc.push(part);
        Ok(acc)
    });

    let mut result = result?;
    result.remove(0);
    Ok(result)
}

async fn download_text(driver: &WebDriver) -> anyhow::Result<String> {
    driver
        .goto("https://www.facebook.com/Emaliazablocie")
        .await?;

    sleep(Duration::from_millis(2000)).await;
    driver.execute("window.scrollBy(0, 1000);", vec![]).await?;

    // Find button to expand the description
    let display_more = driver
        .query(By::XPath(
            &[
                "/",
                &search("L U N C H  M E N U"),
                "/../../",
                &search("Zobacz więcej"),
            ]
            .join(""),
        ))
        .wait(Duration::from_secs(200), Duration::from_millis(100))
        .first()
        .await?;

    // Scroll to the button
    display_more.scroll_into_view().await?;
    // Scroll up, because top bar obscures the button
    driver.execute("window.scrollBy(0, -200);", vec![]).await?;
    // Expand the description
    driver
        .action_chain()
        .move_to_element_center(&display_more)
        .click()
        .perform()
        .await?;

    // Get the text
    let menu_text = driver
        .query(By::XPath(
            &["/", &search("L U N C H  M E N U"), "/../.."].join(""),
        ))
        .wait(Duration::from_secs(200), Duration::from_millis(500))
        .first()
        .await?;

    Ok(menu_text.text().await?)
}

#[cfg(test)]
mod test {
    #[test]
    fn emalia_parse() {
        let result = super::parse_menu(
            r"L U N C H  M E N U 
  13 – 17.03
Od poniedziałku do piątku zjecie u nas pyszny, dwudaniowy obiad, zupę i drugie danie – w wariancie klasycznym lub wege. 12:00 – 16:00 (lub do wyczerpania zapasów), za 29 zł / 31 na wynos.
PONIEDZIAŁEK
 Zupa pomidorowa z ryżem
 Cheeseburger w bułce z Arcymonka, sałata masłowa, sos koktajlowy, pomidor i pieczone ziemniaczki
 Burger z pieczonym burakiem w bułce z Arcymonka, ser feta, słonecznik, salata masłowa i pieczone ziemniaczki
WTOREK
 Krem marchewki i pomarańczy i prażone wiórki kokosa
 Kebab w tortilli z marynowanym kurczakiem po grecku, sałata masłowa, sos tzatziki, czerwona cebula i pieczone ziemniaczki
 Kebab w tortilli z halloumi z grilla, sałata masłowa, czerwona cebula, sos tzatziki i pieczone ziemniaczki
ŚRODA
 Zupa pieczarkowa z makaronem
 Bitki wieprzowe w sosie własnym, kluski śląskie domowej roboty i surówka z buraczków z chrzanem
 Risotto grzybowe, parmezan, oliwa truflowa i rukola
CZWARTEK
 Krem z batata z chilli posypany pokruszonym twarogiem
 Nuggetsy z kurczaka, frytki, sos czosnkowy domowej roboty i colesław z białej kapusty
 Sałatka z falafelem, granatem, ogórkiem i pomidorkami
PIĄTEK
 Czeska zupa czosnkowa na boczku i grzanka czosnkowa
 Spaghetti z klopsikami wołowymi w sosie pomidorowym, ser coreggio i oregano
 Makaron z szpinakiem, suszonymi pomidorami, czosnkiem i parmezanem
Rezerwujcie stoliki i do zobaczenia w Emalii!
 578 364 376
 emaliarezerwacje@gmail.com",
        );

        let Ok(result) = result else {
            panic!("Result is error")
        };

        assert!(result == [
            [
                "Zupa pomidorowa z ryżem",
                "Cheeseburger w bułce z Arcymonka, sałata masłowa, sos koktajlowy, pomidor i pieczone ziemniaczki",
                "Burger z pieczonym burakiem w bułce z Arcymonka, ser feta, słonecznik, salata masłowa i pieczone ziemniaczki",
            ],
            [
                "Krem marchewki i pomarańczy i prażone wiórki kokosa",
                "Kebab w tortilli z marynowanym kurczakiem po grecku, sałata masłowa, sos tzatziki, czerwona cebula i pieczone ziemniaczki",
                "Kebab w tortilli z halloumi z grilla, sałata masłowa, czerwona cebula, sos tzatziki i pieczone ziemniaczki",
            ],
            [
                "Zupa pieczarkowa z makaronem",
                "Bitki wieprzowe w sosie własnym, kluski śląskie domowej roboty i surówka z buraczków z chrzanem",
                "Risotto grzybowe, parmezan, oliwa truflowa i rukola",
            ],
            [
                "Krem z batata z chilli posypany pokruszonym twarogiem",
                "Nuggetsy z kurczaka, frytki, sos czosnkowy domowej roboty i colesław z białej kapusty",
                "Sałatka z falafelem, granatem, ogórkiem i pomidorkami",
            ],
            [
                "Czeska zupa czosnkowa na boczku i grzanka czosnkowa",
                "Spaghetti z klopsikami wołowymi w sosie pomidorowym, ser coreggio i oregano",
                "Makaron z szpinakiem, suszonymi pomidorami, czosnkiem i parmezanem",
            ],
        ]);
    }
}
