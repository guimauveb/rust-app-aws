use serde::Deserialize;

#[derive(Deserialize)]
pub enum Status {
    Success,
    Error,
    Unknown,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct IArticle {
    pub id: i32,
    pub title: String,
    pub pub_date: String,
    pub published: bool,
    pub preview: String,
    pub image: String,
    pub content: String,
}
