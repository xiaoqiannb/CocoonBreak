use anyhow::Result;
use scraper::{Html, Selector};
use crate::models::Video;
use crate::browser::HeadlessBrowser;

pub struct BilibiliClient {
    browser: HeadlessBrowser,
}

impl BilibiliClient {
    pub async fn new() -> Result<Self> {
        let browser = HeadlessBrowser::new().await?;
        Ok(BilibiliClient { browser })
    }

    pub async fn fetch_recommendations(&self) -> Result<Vec<Video>> {
        let url = "https://www.bilibili.com";
        let html = self.browser.navigate_to_url(url).await?;
        
        let document = Html::parse_document(&html);
        let mut videos = Vec::new();
        
        let video_selector = Selector::parse(".bili-video-card").unwrap();
        let title_selector = Selector::parse(".bili-video-card__info--tit").unwrap();
        let author_selector = Selector::parse(".bili-video-card__info--author").unwrap();
        let link_selector = Selector::parse("a.bili-video-card__cover").unwrap();
        let cover_selector = Selector::parse(".bili-video-card__cover img").unwrap();
        let stats_selector = Selector::parse(".bili-video-card__stats--item").unwrap();
        
        for element in document.select(&video_selector) {
            let title = element
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            
            let author = element
                .select(&author_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            
            let url = element
                .select(&link_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| {
                    if href.starts_with("//") {
                        format!("https:{}", href)
                    } else if href.starts_with("/") {
                        format!("https://www.bilibili.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            
            let cover = element
                .select(&cover_selector)
                .next()
                .and_then(|el| el.value().attr("src"))
                .map(|src| {
                    if src.starts_with("//") {
                        format!("https:{}", src)
                    } else {
                        src.to_string()
                    }
                })
                .unwrap_or_default();
            
            let mut views = String::new();
            let mut duration = String::new();
            
            for stat in element.select(&stats_selector) {
                let text = stat.text().collect::<String>();
                if text.contains("播放") {
                    views = text.replace("播放", "").trim().to_string();
                }
            }
            
            if !title.is_empty() && !url.is_empty() {
                videos.push(Video {
                    title,
                    author,
                    url,
                    cover,
                    duration,
                    views,
                    description: String::new(),
                });
            }
        }
        
        Ok(videos)
    }

    pub async fn search_videos(&self, keyword: &str) -> Result<Vec<Video>> {
        let url = format!("https://search.bilibili.com/all?keyword={}", 
                         urlencoding::encode(keyword));
        let html = self.browser.navigate_to_url(&url).await?;
        
        let document = Html::parse_document(&html);
        let mut videos = Vec::new();
        
        let video_selector = Selector::parse(".bili-video-card").unwrap();
        let title_selector = Selector::parse(".bili-video-card__info--tit").unwrap();
        let author_selector = Selector::parse(".bili-video-card__info--author").unwrap();
        let link_selector = Selector::parse("a.bili-video-card__cover").unwrap();
        let cover_selector = Selector::parse(".bili-video-card__cover img").unwrap();
        
        for element in document.select(&video_selector) {
            let title = element
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            
            let author = element
                .select(&author_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            
            let url = element
                .select(&link_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| {
                    if href.starts_with("//") {
                        format!("https:{}", href)
                    } else if href.starts_with("/") {
                        format!("https://www.bilibili.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            
            let cover = element
                .select(&cover_selector)
                .next()
                .and_then(|el| el.value().attr("src"))
                .map(|src| {
                    if src.starts_with("//") {
                        format!("https:{}", src)
                    } else {
                        src.to_string()
                    }
                })
                .unwrap_or_default();
            
            if !title.is_empty() && !url.is_empty() {
                videos.push(Video {
                    title,
                    author,
                    url,
                    cover,
                    duration: String::new(),
                    views: String::new(),
                    description: String::new(),
                });
            }
        }
        
        Ok(videos)
    }

    pub async fn watch_video(&self, url: &str) -> Result<()> {
        self.browser.navigate_to_url(url).await?;
        
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        
        Ok(())
    }
}