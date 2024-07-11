use log::info;

use crate::config::Config;
use crate::db::Database;

pub struct Handler {
    db: Option<Database>,
    config: Config,
}

impl Handler {
    pub fn new() -> Self {
        let config = Config::new();
        if config.db_path().exists() {
            return Handler {
                db: Some(Database::open(config.db_path())),
                config,
            };
        }
        info!("No database exists yet");
        Handler { db: None, config }
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.config.tags.clone()
    }

    pub fn get_folder(&self) -> String {
        self.config.folder.clone()
    }
}
