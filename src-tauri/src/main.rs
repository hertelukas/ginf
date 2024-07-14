// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db;
pub mod models;
pub mod schema;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use log::info;
use tauri::{Manager, State};

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[tauri::command]
fn add_tag(tag: &str) -> Result<(), String> {
    info!("Adding tag {tag}");
    Ok(())
}

#[tauri::command]
fn get_tags(pool: State<DbPool>) -> Result<Vec<String>, String> {
    info!("Loading tags");
    Ok(db::get_tags(&pool))
}

#[tauri::command]
fn import(path: String, tags: Vec<String>) {
    info!("Importing file {:?} with tags {:?}", path, tags);
}

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let config = config::Config::new();
            let db_pool = db::establish_connection_pool(config.db_path());
            db::run_migrations(&db_pool);
            info!("Running in {}", config.folder);
            app.manage(db_pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_tag, get_tags, import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
