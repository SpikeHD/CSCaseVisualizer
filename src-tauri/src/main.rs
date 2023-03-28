// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod inventory;
mod dumps;

fn main() {
    // Create dumps folder if it doesnt exist
    if !std::path::Path::new("dumps").exists() {
        std::fs::create_dir("dumps").unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            inventory::get_main,
            dumps::list_dumps,
            dumps::get_dump
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
