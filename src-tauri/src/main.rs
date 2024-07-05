// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod handler;
mod config;
use std::sync::Mutex;

use log::info;
use handler::Handler;
use tauri::Manager;

#[tauri::command]
fn add_tag(tag: &str) -> Result<(), String> {
    info!("Adding tag {tag}");
    Ok(())
}

#[tauri::command]
fn get_tags() -> Result<Vec<String>, String> {
    Ok(Vec::<String>::new())
}

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let mgr = Handler::new();
            app.manage(Mutex::new(mgr));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_tag, get_tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
