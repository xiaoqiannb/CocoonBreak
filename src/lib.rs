mod browser;
mod bilibili;
mod llm;
mod models;
mod commands;

use commands::init_app_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = init_app_state();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            fetch_recommendations,
            analyze_content,
            generate_reverse_search,
            search_and_watch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}