use std::path::PathBuf;

use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn open(path: PathBuf) -> Self {
        let mut conn = SqliteConnection::establish(&path.to_string_lossy())
            .unwrap_or_else(|_| panic!("Error connecitng to {}", &path.to_string_lossy()));

        info!("Running database migrations...");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Migration failed");

        Database { connection: conn }
    }
}
