use crate::config::Config;
use crate::db::Database;

pub struct Handler {
    db: Option<Database>,
    config: Config,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            db: None,
            config: Config::new(),
        }
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.config.tags.clone()
    }
}
