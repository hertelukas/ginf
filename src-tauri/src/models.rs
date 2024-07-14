use diesel::prelude::*;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i32,
    pub path: String,
    pub name: String,
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}


#[derive(Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(File))]
#[diesel(primary_key(file_id, tag_id))]
#[diesel(table_name = crate::schema::file_tags)]
pub struct FileTags {
    pub rowid: i32,
    pub file_id: i32,
    pub tag_id: i32
}
