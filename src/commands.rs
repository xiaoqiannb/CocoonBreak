use crate::bilibili::BilibiliClient;
use crate::llm::LLMClient;
use crate::models::{ContentAnalysis, ReverseSearch, SearchProgress, Video};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    bilibili_client: Mutex<Option<BilibiliClient>>,
    llm_client: Mutex<Option<LLMClient>>,
}

#[tauri::command]
pub async fn fetch_recommendations(
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String> {
    let mut client_guard = state.bilibili_client.lock().unwrap();
    
    if client_guard.is_none() {
        *client_guard = Some(BilibiliClient::new().await.map_err(|e| e.to_string())?);
    }
    
    let client = client_guard.as_ref().unwrap();
    let videos = client.fetch_recommendations().await.map_err(|e| e.to_string())?;
    
    Ok(videos)
}

#[tauri::command]
pub async fn analyze_content(
    videos: Vec<Video>,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<ContentAnalysis, String> {
    let mut client_guard = state.llm_client.lock().unwrap();
    
    if client_guard.is_none() {
        *client_guard = Some(LLMClient::new(api_key));
    }
    
    let client = client_guard.as_ref().unwrap();
    let analysis = client
        .analyze_content(&videos)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(analysis)
}

#[tauri::command]
pub async fn generate_reverse_search(
    analysis: ContentAnalysis,
    state: State<'_, AppState>,
) -> Result<ReverseSearch, String> {
    let client_guard = state.llm_client.lock().unwrap();
    let client = client_guard.as_ref().ok_or("LLM client not initialized")?;
    
    let search = client
        .generate_reverse_search(&analysis)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(search)
}

#[tauri::command]
pub async fn search_and_watch(
    search_terms: Vec<String>,
    api_key: String,
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), String> {
    let mut client_guard = state.bilibili_client.lock().unwrap();
    
    if client_guard.is_none() {
        *client_guard = Some(BilibiliClient::new().await.map_err(|e| e.to_string())?);
    }
    
    let client = client_guard.as_ref().unwrap();
    
    for (index, term) in search_terms.iter().enumerate() {
        let progress = (index as f64) / (search_terms.len() as f64) * 100.0;
        
        window
            .emit("search-progress", SearchProgress {
                current_term: term.clone(),
                progress,
                videos_found: Vec::new(),
                status: format!("正在搜索: {}", term),
            })
            .map_err(|e| e.to_string())?;
        
        let videos = client.search_videos(term).await.map_err(|e| e.to_string())?;
        
        window
            .emit("search-progress", SearchProgress {
                current_term: term.clone(),
                progress,
                videos_found: videos.clone(),
                status: format!("找到 {} 个视频", videos.len()),
            })
            .map_err(|e| e.to_string())?;
        
        if let Some(video) = videos.first() {
            window
                .emit("search-progress", SearchProgress {
                    current_term: term.clone(),
                    progress,
                    videos_found: videos,
                    status: format!("正在观看: {}", video.title),
                })
                .map_err(|e| e.to_string())?;
            
            client.watch_video(&video.url).await.map_err(|e| e.to_string())?;
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    window
        .emit("search-complete", "搜索和观看完成")
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn init_app_state() -> AppState {
    AppState {
        bilibili_client: Mutex::new(None),
        llm_client: Mutex::new(None),
    }
}