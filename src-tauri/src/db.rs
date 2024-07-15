use std::path::PathBuf;

use crate::models::*;
use crate::DbPool;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_foreign_keys: bool,
}

// Taken from https://stackoverflow.com/a/57717533/9820072
impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn establish_connection_pool(path: PathBuf) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(path.to_string_lossy());
    Pool::builder()
        .connection_customizer(Box::new(ConnectionOptions {
            enable_foreign_keys: true,
        }))
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn run_migrations(pool: &DbPool) {
    info!("Running migrations...");
    pool.get()
        .expect("Could not get connection for migrations")
        .run_pending_migrations(MIGRATIONS)
        .expect("Migration failed");
}

pub fn get_tags(pool: &DbPool) -> Result<Vec<String>, String> {
    use crate::schema::tags::dsl::*;
    let connection = &mut pool.get().unwrap();

    let results = &tags
        .select(Tag::as_select())
        .load(connection)
        .expect("Error loading tags");

    info!(
        "Loaded tags: {:?}",
        results
            .iter()
            .map(|t| t.tag.clone())
            .collect::<Vec<String>>()
    );

    Ok(results
        .iter()
        .map(|t| t.tag.clone())
        .collect::<Vec<String>>())
}

pub fn get_tag(tag_name: &String, pool: &DbPool) -> Result<Tag, String> {
    use crate::schema::tags::dsl::*;
    let connecton = &mut pool.get().unwrap();

    let result = tags
        .filter(tag.eq(tag_name))
        .select(Tag::as_select())
        .first(connecton)
        .expect("Error loading tag");

    Ok(result)
}

pub fn insert_file(
    relative_path: &String,
    name: &String,
    tags: &Vec<String>,
    pool: &DbPool,
) -> Result<(), String> {
    use crate::schema::file_tags;
    use crate::schema::files;
    let connection = &mut pool.get().unwrap();

    // Somewhat ugly: We just ignore when we fail to get a tag
    let tags: Vec<Tag> = tags.iter().filter_map(|t| get_tag(t, pool).ok()).collect();

    let file = diesel::insert_into(files::table)
        .values((files::path.eq(relative_path), files::name.eq(name)))
        .returning(File::as_returning())
        .get_result(connection)
        .expect("Failed to insert file");

    for tag in tags {
        diesel::insert_into(file_tags::table)
            .values((file_tags::file_id.eq(file.id), file_tags::tag_id.eq(tag.id)))
            .execute(connection)
            .expect("Failed to isnert file_tag");
    }

    Ok(())
}
