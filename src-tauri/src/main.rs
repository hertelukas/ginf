// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db;
mod handler;
use std::sync::Mutex;

use handler::Handler;
use log::info;
use tauri::{Manager, State};

#[tauri::command]
fn add_tag(tag: &str) -> Result<(), String> {
    info!("Adding tag {tag}");
    Ok(())
}

#[tauri::command]
fn get_tags(handler: State<Mutex<Handler>>) -> Result<Vec<String>, String> {
    match handler.lock() {
        Ok(hd) => Ok(hd.get_tags()),
        Err(_) => Err("Could not lock handler".to_string()),
    }
}

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let handler = Handler::new();
            app.manage(Mutex::new(handler));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_tag, get_tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
