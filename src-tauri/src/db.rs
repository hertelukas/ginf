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
use log::warn;

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

    let results = match tags.select(Tag::as_select()).load(connection) {
        Ok(res) => res,
        Err(e) => {
            warn!("Getting tags failed: {:?}", e);
            return Err(format!("Getting tags failed."));
        }
    };

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

    let result = match tags
        .filter(tag.eq(tag_name))
        .select(Tag::as_select())
        .first(connecton)
    {
        Ok(res) => res,
        Err(e) => {
            warn!("Getting tag {tag_name} failed: {:?}", e);
            return Err(format!("Getting {tag_name} failed."));
        }
    };

    Ok(result)
}

// TODO this needs some testing
pub fn get_files(pool: &DbPool) -> Result<Vec<(File, Vec<Tag>)>, String> {
    use crate::schema::files;
    use crate::schema::tags;
    let connection = &mut pool.get().unwrap();

    let all_files: Vec<File> = files::table
        .select(File::as_select())
        .load(connection)
        .unwrap();

    let all_used_tags: Vec<(FileTags, Tag)> = FileTags::belonging_to(&all_files)
        .inner_join(tags::table)
        .select((FileTags::as_select(), Tag::as_select()))
        .load(connection)
        .unwrap();

    let grouped_tags = all_used_tags.grouped_by(&all_files);

    let tags_per_file: Vec<(File, Vec<Tag>)> = all_files
        .into_iter()
        .zip(grouped_tags)
        .map(|(file, file_tags)| {
            let tags: Vec<Tag> = file_tags.into_iter().map(|(_, tag)| tag).collect();
            (file, tags)
        })
        .collect();

    Ok(tags_per_file)
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

    let file = match diesel::insert_into(files::table)
        .values((files::path.eq(relative_path), files::name.eq(name)))
        .returning(File::as_returning())
        .get_result(connection)
    {
        Ok(res) => res,
        Err(e) => {
            warn!("Inserting file {name} failed: {:?}", e);
            return Err(format!("Inserting {name} failed."));
        }
    };

    for tag in tags {
        match diesel::insert_into(file_tags::table)
            .values((file_tags::file_id.eq(file.id), file_tags::tag_id.eq(tag.id)))
            .execute(connection)
        {
            Ok(_) => (),
            Err(e) => {
                warn!("Tagging file {name} with {} failed: {:?}", tag.tag, e);
                return Err(format!("Tagging {name} with {} failed.", tag.tag));
            }
        };
    }

    Ok(())
}
