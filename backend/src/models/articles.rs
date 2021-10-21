use {
    crate::schema::*,
    serde_derive::{Deserialize, Serialize},
};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub preview: String,
    pub image: String,
    pub content: String,
}
