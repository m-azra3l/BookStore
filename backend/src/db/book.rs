use super::{get_db_con, Result};
use crate::{error::Error::*, DBPool};
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "book";
const SELECT_FIELDS: &str = "id, author_id, title, genre";

pub async fn fetch(db_pool: &DBPool, author_id: i32) -> Result<Vec<Book>> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "SELECT {} FROM {} WHERE author_id = $1",
        SELECT_FIELDS, TABLE
    );
    let rows = con
        .query(query.as_str(), &[&author_id])
        .await
        .map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_book(&r)).collect())
}

pub async fn create(db_pool: &DBPool, author_id: i32, body: BookRequest) -> Result<Book> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (title, author_id, genre) VALUES ($1, $2, $3) RETURNING *",
        TABLE
    );
    let row = con
        .query_one(
            query.as_str(),
            &[&body.title, &author_id, &body.genre],
        )
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_book(&row))
}

pub async fn delete(db_pool: &DBPool, author_id: i32, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1 AND author_id = $2", TABLE);
    con.execute(query.as_str(), &[&id, &author_id])
        .await
        .map_err(DBQueryError)
}

fn row_to_book(row: &Row) -> Book {
    let id: i32 = row.get(0);
    let author_id: i32 = row.get(1);
    let title: String = row.get(2);
    let genre: String = row.get(3);
    Book {
        id,
        title,
        author_id,
        genre,
    }
}