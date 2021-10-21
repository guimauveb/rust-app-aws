use {
    serde_derive::{Deserialize, Serialize},
    std::fmt::Debug,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IArticle {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub preview: String,
    pub image: String,
    pub content: String,
}
