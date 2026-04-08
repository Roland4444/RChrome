// use headless_chrome::{Browser, LaunchOptions};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let options = LaunchOptions::default_builder()
//         .headless(false)
//         .build()
//         .unwrap();

//     let browser = Browser::new(options)?; 
//     let tab = browser.new_tab()?;
//     tab.navigate_to("https://www.google.com")?;
//     tab.wait_until_navigated()?;    
//     std::thread::sleep(std::time::Duration::from_secs(15));

//     Ok(())
// }



use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:55671", caps).await?; // chromedriver по умолчанию

    let init_addr = "https://relits.bitrix24.ru/company/personal/user/";
    let id = 1;
    driver.get(&format!("{}{}", init_addr, id)).await?;
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    Ok(())
}