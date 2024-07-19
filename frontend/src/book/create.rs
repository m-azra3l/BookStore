use common::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    prelude::*,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub author_id: i32,
}

pub struct CreateForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_book_title: String,
    state_genre: String,
}

impl CreateForm {
    fn render_form(&self, author_id: i32) -> Html {
        let edit_title = self
            .link
            .callback(move |e: InputData| Msg::EditTitle(e.value));
        let edit_genre = self.link.callback(move |e: ChangeData| match e {
            ChangeData::Select(elem) => Msg::EditGenre(elem.value()),
            _ => unreachable!("only used on select field"),
        });

        html! {
            <div class=classes!("book-form")>
                <div>
                    <input type="text" value={self.state_book_title.clone()} oninput={edit_title} />
                </div>
                <div>
                    <select onchange={edit_genre}>
                        <option value="romance" selected=true>{ "Romance" }</option>
                        <option value="fiction">{ "Fiction" }</option>
                    </select>
                </div>
                <div>
                    <button onclick=self.link.callback(move |_| Msg::MakeReq(author_id))>{"Submit"}</button>
                </div>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<BookResponse, anyhow::Error>),
    EditTitle(String),
    EditGenre(String),
}

impl Component for CreateForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state_book_title: String::new(),
            state_genre: String::from("romance"),
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form(self.props.author_id) }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let body = BookRequest {
                    title: self.state_book_title.clone(),
                    genre: self.state_genre.clone(),
                };
                let req = Request::post(&format!("http://localhost:8000/author/{}/book", id))
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<BookResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("book created: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: format!("/app/{}", self.props.author_id),
                        state: (),
                    }));
                }
            }
            Msg::EditTitle(input) => {
                self.state_book_title = input;
            }
            Msg::EditGenre(input) => {
                ConsoleService::info(&format!("input: {:?}", input));
                self.state_genre = input;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}