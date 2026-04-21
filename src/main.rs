//use headless_chrome::{Browser, LaunchOptions};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let options = LaunchOptions::default_builder()
//         .headless(true)
//         .build()
//         .unwrap();

//     let browser = Browser::new(options)?; 
//     let tab = browser.new_tab()?;
//     tab.navigate_to("https://www.google.com")?;
//     tab.wait_until_navigated()?;    
//     std::thread::sleep(std::time::Duration::from_secs(15));

//     Ok(())
// }



// use thirtyfour::prelude::*;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let caps = DesiredCapabilities::chrome();
//     let driver = WebDriver::new("http://localhost:55671", caps).await?; // chromedriver по умолчанию

//     let init_addr = "https://relits.bitrix24.ru/company/personal/user/";
//     let id = 1;
//     driver.get(&format!("{}{}", init_addr, id)).await?;
//     tokio::time::sleep(std::time::Duration::from_secs(30)).await;

//     Ok(())
// }


use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless")?;
    caps.add_arg("--disable-gpu")?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--window-size=1920,1080")?;

    let driver = WebDriver::new("http://localhost:21000", caps).await?;

    driver.get("https://duckduckgo.com").await?;
    sleep(Duration::from_secs(2)).await;

    let search_box = driver.find(By::Name("q")).await?;
    search_box.send_keys("Rust programming language").await?;
    search_box.send_keys(Key::Enter).await?;

    // Вместо нерабочего селектора просто ждём 3 секунды
    sleep(Duration::from_secs(3)).await;

    println!("Title: {:?}", driver.title().await?);

    driver.quit().await?;
    Ok(())
}