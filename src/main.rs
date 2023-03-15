mod common;
mod emalia;
mod talerz;

use common::fb_accept_cookies;
use emalia::run_emalia;
use talerz::run_talerz;
use tera::Context;
use thirtyfour::{prelude::*, CapabilitiesHelper};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = fs::create_dir("_site").await;

    let mut caps = DesiredCapabilities::firefox();
    if true {
        caps.set_headless()?;
        caps.add_firefox_arg("--disable-gpu")?;
    }

    let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

    if let Err(err) = run_restaurants(&driver).await {
        driver.quit().await?;
        Err(err)
    } else {
        driver.quit().await?;
        Ok(())
    }
}

async fn run_restaurants(driver: &WebDriver) -> anyhow::Result<()> {
    fb_accept_cookies(driver).await?;
    let data = run_emalia(driver).await?;
    let image_path = run_talerz(driver).await?;

    let mut context = Context::new();
    context.insert("emalia", &data);
    context.insert("talerz_img", image_path);
    let html = String::from_utf8(fs::read("index.html").await?)?;
    let output = tera::Tera::default().render_str(&html, &context)?;
    fs::write("_site/index.html", output).await?;

    Ok(())
}
