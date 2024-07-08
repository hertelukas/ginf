use crate::config::Config;
use crate::db::Database;

pub struct Handler {
    db: Database,
    config: Config,
}

impl Handler {
    pub fn new() -> Self {
        let config = Config::new();
        let db = Database::new(config.db_path());
        db.create_tables();
        Handler { db, config }
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.config.tags.clone()
    }
}
