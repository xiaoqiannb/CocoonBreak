use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use std::time::Duration;

pub struct HeadlessBrowser {
    browser: Browser,
}

impl HeadlessBrowser {
    pub async fn new() -> Result<Self> {
        let browser = Browser::new(
            LaunchOptions::default()
                .headless(true)
                .args(vec![
                    "--disable-gpu",
                    "--no-sandbox",
                    "--disable-dev-shm-usage",
                    "--disable-web-security",
                    "--disable-features=IsolateOrigins,site-per-process",
                ])
        )?;
        
        Ok(HeadlessBrowser { browser })
    }

    pub async fn navigate_to_url(&self, url: &str) -> Result<String> {
        let tab = self.browser.new_tab()?;
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;
        
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        let html = tab.get_content()?;
        Ok(html)
    }

    pub async fn execute_script(&self, script: &str) -> Result<String> {
        let tab = self.browser.new_tab()?;
        let result = tab.evaluate(script, false)?;
        
        Ok(result.value.to_string())
    }

    pub async fn screenshot(&self, url: &str) -> Result<Vec<u8>> {
        let tab = self.browser.new_tab()?;
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;
        
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        let png_data = tab.capture_screenshot(
            headless_chrome::protocol::page::ScreenshotFormat::PNG,
            None,
            true,
            None,
        )?;
        
        Ok(png_data)
    }
}

impl Default for HeadlessBrowser {
    fn default() -> Self {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { Self::new().await.unwrap() })
    }
}