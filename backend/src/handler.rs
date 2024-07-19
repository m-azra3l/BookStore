use crate::{db, DBPool, Result};
use common::*;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn list_books_handler(author_id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let books = db::book::fetch(&db_pool, author_id)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &books.into_iter().map(BookResponse::of).collect(),
    ))
}

pub async fn create_book_handler(
    author_id: i32,
    body: BookRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&BookResponse::of(
        db::book::create(&db_pool, author_id, body)
            .await
            .map_err(reject::custom)?,
    )))
}

pub async fn delete_book_handler(author_id: i32, id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::book::delete(&db_pool, author_id, id)
        .await
        .map_err(reject::custom)?;
    Ok(StatusCode::OK)
}

pub async fn list_authors_handler(db_pool: DBPool) -> Result<impl Reply> {
    let authors = db::author::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &authors.into_iter().map(AuthorResponse::of).collect(),
    ))
}

pub async fn fetch_author_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let author = db::author::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&AuthorResponse::of(author)))
}

pub async fn create_author_handler(body: AuthorRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&AuthorResponse::of(
        db::author::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}