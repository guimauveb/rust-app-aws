use {
    crate::{interfaces::IArticle, models::articles::Article, schema::articles, Pool, API_URL},
    actix_web::{web, Error, HttpResponse},
    diesel::prelude::*,
};

// /articles/{id}/
pub fn db_get_article_by_id(
    pool: web::Data<Pool>,
    article_pk: i32,
) -> Result<IArticle, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let article = articles::table
        .find(article_pk)
        .first::<Article>(&conn)
        .expect("Article not found.");

    Ok(IArticle {
        id: article.id,
        title: article.title,
        pub_date: article.pub_date,
        published: article.published,
        preview: article.preview,
        image: API_URL.to_owned() + &article.image,
        content: article.content,
    })
}
pub async fn get_article_by_id(
    pool: web::Data<Pool>,
    article_pk: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_article_by_id(pool, article_pk.into_inner()))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// /articles/
fn db_get_all_articles(pool: web::Data<Pool>) -> Result<Vec<Article>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let articles = articles::table
        .order_by(articles::pub_date.desc())
        .load::<Article>(&conn)
        .expect("Could not load articles.");

    Ok(articles)
}
pub async fn get_all_articles(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_all_articles(pool))
        .await
        .map(|articles| HttpResponse::Ok().json(articles))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
