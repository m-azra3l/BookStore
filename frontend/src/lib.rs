#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod author;
mod book;

pub type Anchor = RouterAnchor<AppRoute>;

struct FullStackApp {}

pub enum Msg {}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/app/create-author"]
    CreateAuthor,
    #[to = "/app/create-book/{id}"]
    CreateBook(i32),
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

impl Component for FullStackApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("app")>
                <div class=classes!("nav")>
                    <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::CreateAuthor => {
                                    html! {
                                        <div>
                                            <author::create::CreateForm />
                                        </div>}
                                }
                                AppRoute::CreateBook(author_id) => {
                                    html! {
                                        <div>
                                            <book::create::CreateForm author_id=author_id/>
                                        </div>}
                                }
                                AppRoute::Detail(author_id) => {
                                    html! {
                                        <div>
                                            <author::detail::Detail author_id=author_id/>
                                        </div>}
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <author::list::List />
                                            <br />
                                            <Anchor route=AppRoute::CreateAuthor>
                                            { "Create New Author" }
                                                </Anchor>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<FullStackApp>::new().mount_to_body();
}