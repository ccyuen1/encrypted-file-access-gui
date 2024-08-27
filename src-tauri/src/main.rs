// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod create;
mod open;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create::create, open::open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
