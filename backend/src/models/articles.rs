use {
    super::from_model::FromModel,
    crate::{schema::articles, API_URL},
    diesel::{PgConnection, QueryDsl, RunQueryDsl},
    serde::Serialize,
};

#[derive(Identifiable, Debug, Serialize, Queryable, Clone, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ArticleRepresentation {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}

impl FromModel<Article> for ArticleRepresentation {
    fn from_model(article: Article) -> ArticleRepresentation {
        ArticleRepresentation {
            id: article.id,
            title: article.title,
            pub_date: article.pub_date,
            published: article.published,
            headline: article.headline,
            image: API_URL.to_owned() + &article.image,
            content: article.content,
        }
    }
}

impl Article {
    pub fn get(
        id: &i32,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(ArticleRepresentation::from_model(article))
    }

    pub fn list(
        connection: &PgConnection,
    ) -> Result<Vec<ArticleRepresentation>, diesel::result::Error> {
        let articles = articles::table
            .load::<Article>(connection)
            .expect("Could not load articles.");
        let results = articles
            .into_iter()
            .map(ArticleRepresentation::from_model)
            .collect();

        Ok(results)
    }
}
