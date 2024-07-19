use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AuthorRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AuthorResponse {
    pub id: i32,
    pub name: String,
}

impl AuthorResponse {
    pub fn of(author: Author) -> AuthorResponse {
        AuthorResponse {
            id: author.id,
            name: author.name,
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub genre: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BookRequest {
    pub title: String,
    pub genre: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BookResponse {
    pub id: i32,
    pub title: String,
    pub genre: String,
}

impl BookResponse {
    pub fn of(book: Book) -> BookResponse {
        BookResponse {
            id: book.id,
            title: book.title,
            genre: book.genre,
        }
    }
}