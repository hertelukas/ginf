use crate::db::Database;
use crate::config::Config;

pub struct Handler {
    db: Option<Database>,
    config: Config,
}

impl Handler {
    pub fn new() -> Self {
        Handler{db: None, config: Config::new()}
    }
}
