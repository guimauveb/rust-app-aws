use {
    crate::{schema::articles, API_URL},
    diesel::{PgConnection, QueryDsl, RunQueryDsl},
    serde::{Deserialize, Serialize},
};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleRepresentation {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}

impl Article {
    fn into_representation(self) -> ArticleRepresentation {
        ArticleRepresentation {
            id: self.id,
            title: self.title,
            pub_date: self.pub_date,
            published: self.published,
            headline: self.headline,
            image: API_URL.to_owned() + &self.image,
            content: self.content,
        }
    }

    pub fn get(
        id: &i32,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(article.into_representation())
    }

    pub fn list(
        connection: &PgConnection,
    ) -> Result<Vec<ArticleRepresentation>, diesel::result::Error> {
        let articles = articles::table
            .load::<Article>(connection)
            .expect("Could not load articles.");
        let results = articles
            .into_iter()
            .map(|article| article.into_representation())
            .collect();

        Ok(results)
    }
}
