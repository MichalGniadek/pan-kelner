use crate::common::search;
use anyhow::Context;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::time::Duration;
use thirtyfour::{prelude::ElementQueryable, By, WebDriver};
use tokio::time::sleep;

pub async fn run_talerz(driver: &WebDriver) -> anyhow::Result<&str> {
    let url = get_image_url(driver).await?;
    let image = download_image(&url).await?;
    let image = transform_image(image)?;

    image.save("_site/transformed.png")?;
    Ok("transformed.png")
}

fn transform_image(image: DynamicImage) -> anyhow::Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = image.grayscale();

    let (w, h) = image.dimensions();
    let mut output = ImageBuffer::new(w, h);

    let g = colorgrad::CustomGradient::new()
        .html_colors(&["#e6ecfe", "#e6ecfe", "#061577", "#061577"])
        .domain(&[0.0, 0.6, 0.8, 1.0])
        .build()?;

    for (x, y, pixel) in image.pixels() {
        let t = pixel.0[0] as f64 / 255.0;
        let value = g.at(t).to_array().map(|f| (f * 255.0) as u8);

        output.put_pixel(x, y, Rgba::<u8>(value));
    }

    Ok(output)
}

async fn download_image(url: &str) -> anyhow::Result<DynamicImage> {
    let data = reqwest::get(url).await?.bytes().await?;

    let image = image::load_from_memory_with_format(&data[..], image::ImageFormat::Jpeg)?;

    Ok(image)
}

fn any(_: &str) -> bool {
    true
}

async fn get_image_url(driver: &WebDriver) -> anyhow::Result<String> {
    driver
        .goto("https://www.facebook.com/talerzrestaurant")
        .await?;

    sleep(Duration::from_millis(2000)).await;
    driver.execute("window.scrollBy(0, 1000);", vec![]).await?;

    // Find button to expand the description
    let image = driver
        .query(By::XPath(
            &[
                "/",
                &search("lunch menu"),
                "/../../../../../../..//img[contains(@alt, 'PONIEDZIAŁEK')]",
            ]
            .join(""),
        ))
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?;

    // Scroll to the image
    image.scroll_into_view().await?;
    // Expand the image
    driver
        .action_chain()
        .move_to_element_center(&image)
        .click()
        .perform()
        .await?;

    sleep(Duration::from_millis(500)).await;

    let expanded_image = driver
        .query(By::XPath("//img[contains(@alt, 'PONIEDZIAŁEK')]"))
        .without_id(image.id().await?.unwrap_or_default())
        .without_attributes(&[("width", any), ("height", any)])
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?;

    let image_url = expanded_image.attr("src").await?.context("No image url")?;

    Ok(image_url)
}
