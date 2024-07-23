// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db;
pub mod models;
pub mod schema;

use std::path::Path;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use log::{debug, info};
use rand::{distributions::Alphanumeric, Rng};
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
    db::get_tags(&pool)
}

#[tauri::command]
fn import(path: String, tags: Vec<String>, pool: State<DbPool>) -> Result<(), String> {
    info!("Importing file {:?} with tags {:?}", path, tags);

    let config = config::Config::new();
    let target_path = config.folder_path();
    let path = Path::new(&path);
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();

    let folder: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let target_path = target_path.join(folder.clone());

    info!("New file location {:?}", target_path);

    // Create target folder
    std::fs::create_dir(&target_path).unwrap();
    let target_path = target_path.join(filename.clone());

    // Copying the file
    match std::fs::copy(&path, &target_path) {
        Ok(bytes) => debug!("Copied {bytes} to the new folder {:?}", target_path),
        Err(error) => return Err("Importing file failed:".to_string() + &error.to_string()),
    }

    db::insert_file(&folder, &filename, &tags, &pool)
}

#[tauri::command]
fn get_files(pool: State<DbPool>) -> Result<Vec<(models::File, Vec<models::Tag>)>, String> {
    info!("Loading files");
    db::get_files(&pool)
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
        .invoke_handler(tauri::generate_handler![add_tag, get_tags, import, get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
