use {
    super::fetch::Fetch,
    crate::{
        entities::interfaces::{IArticle, Status},
        API_URL,
    },
};

pub async fn get_article_list() -> Result<Vec<IArticle>, Status> {
    let url = format!("{}/articles", API_URL);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<Vec<IArticle>>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_article(id: &i32) -> Result<IArticle, Status> {
    let url = format!("{}/articles/{}", API_URL, id);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<IArticle>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
