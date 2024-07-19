use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{
    http::{header, Method},
    Filter, Rejection,
};

mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let book = warp::path!("author" / i32 / "book");
    let book_param = warp::path!("author" / i32 / "book" / i32);
    let author = warp::path("author");

    let book_routes = book
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_books_handler)
        .or(book
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_book_handler))
        .or(book_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_book_handler));

    let author_routes = author
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db_pool.clone()))
        .and_then(handler::fetch_author_handler)
        .or(author
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::list_authors_handler))
        .or(author
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_author_handler));

    let routes = book_routes
        .or(author_routes)
        .recover(error::handle_rejection)
        .with(
            warp::cors()
                .allow_credentials(true)
                .allow_methods(&[
                    Method::OPTIONS,
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                ])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .expose_headers(vec![header::LINK])
                .max_age(300)
                .allow_any_origin(),
        );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}