use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub author_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    books: Option<Vec<BookResponse>>,
    author: Option<AuthorResponse>,
    fetch_books_task: Option<FetchTask>,
    fetch_author_task: Option<FetchTask>,
    delete_book_task: Option<FetchTask>,
}

impl Detail {
    fn render_detail(
        &self,
        author: &Option<AuthorResponse>,
        books: &Option<Vec<BookResponse>>,
    ) -> Html {
        match author {
            Some(o) => {
                html! {
                    <div class=classes!("detail")>
                        <h1>{&o.name}{" ("}<span class=classes!("id")>{o.id}</span>{")"}</h1>
                        {
                            self.view_book_list(books)
                        }

                    <br />
                    <Anchor route=AppRoute::CreateBook(o.id as i32)>
                        { "Create New Book" }
                    </Anchor>
                    </div>
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }

    fn view_book_list(&self, books: &Option<Vec<BookResponse>>) -> Html {
        match books {
            Some(p) => {
                html! {
                    p.iter().map(|book| self.view_book(book)).collect::<Html>()
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }

    fn view_book(&self, book: &BookResponse) -> Html {
        let id = book.id;
        let author_id = self.props.author_id;
        html! {
            <div class=classes!("list-item", "book")>
                <div><b>{ &book.title }</b> { " (" } <button onclick=self.link.callback(move |_| Msg::MakeDeleteBookReq(author_id, id))>{"Delete"}</button> {")"}</div>
                <div>{ &book.genre }</div>
            </div>
        }
    }
}

pub enum Msg {
    MakeBooksReq(i32),
    MakeAuthorReq(i32),
    MakeDeleteBookReq(i32, i32),
    RespBooks(Result<Vec<BookResponse>, anyhow::Error>),
    RespAuthor(Result<AuthorResponse, anyhow::Error>),
    RespDeleteBook(Response<Json<Result<(), anyhow::Error>>>, i32),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeBooksReq(props.author_id));
        link.send_message(Msg::MakeAuthorReq(props.author_id));
        Self {
            props,
            link,
            author: None,
            books: None,
            fetch_books_task: None,
            fetch_author_task: None,
            delete_book_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.author, &self.books)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeBooksReq(id) => {
                let req = Request::get(&format!("http://localhost:8000/author/{}/book", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<BookResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespBooks(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_books_task = Some(task);
                ()
            }
            Msg::MakeAuthorReq(id) => {
                let req = Request::get(&format!("http://localhost:8000/author/{}", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<AuthorResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespAuthor(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_author_task = Some(task);
                ()
            }
            Msg::MakeDeleteBookReq(author_id, book_id) => {
                let req = Request::delete(&format!(
                    "http://localhost:8000/author/{}/book/{}",
                    author_id, book_id
                ))
                .body(Nothing)
                .expect("can make req to backend");

                let cb = self.link.callback(
                    move |response: Response<Json<Result<(), anyhow::Error>>>| {
                        Msg::RespDeleteBook(response, book_id)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.delete_book_task = Some(task);
                ()
            }
            Msg::RespBooks(resp) => {
                if let Ok(data) = resp {
                    self.books = Some(data);
                }
            }
            Msg::RespAuthor(resp) => {
                if let Ok(data) = resp {
                    self.author = Some(data);
                }
            }
            Msg::RespDeleteBook(resp, id) => {
                if resp.status().is_success() {
                    self.books = self
                        .books
                        .as_ref()
                        .map(|books| books.into_iter().filter(|p| p.id != id).cloned().collect());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}