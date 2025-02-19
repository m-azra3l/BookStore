use super::{get_db_con, Result};
use crate::{error::Error::*, DBPool};
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "author";
const SELECT_FIELDS: &str = "id, name";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Author>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_author(&r)).collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: i32) -> Result<Author> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", SELECT_FIELDS, TABLE);

    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_author(&row))
}

pub async fn create(db_pool: &DBPool, body: AuthorRequest) -> Result<Author> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_author(&row))
}

fn row_to_author(row: &Row) -> Author {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    Author { id, name }
}