use std::time::Duration;
use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time::sleep;

pub fn search(text: &str) -> String {
    format!("/*[text()[contains(.,'{text}')]]")
}

pub async fn fb_accept_cookies(driver: &WebDriver) -> anyhow::Result<()> {
    driver.goto("https://www.facebook.com").await?;

    driver
        .query(By::XPath(
            &["/", &search("Zezwól tylko na niezbędne pliki cookie")].join(""),
        ))
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?
        .click()
        .await?;

    sleep(Duration::from_millis(500)).await;

    Ok(())
}
