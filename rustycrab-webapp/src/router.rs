use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/post/:id")] Post {
        id: String,
    },
    #[at("/*path")] Misc {
        path: String,
    },
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {
                <Home />
            },
        MainRoute::Post { id } => html! { <p>{format!("You are looking at Post {}", id)}</p> },
        MainRoute::Misc { path } => html! { <p>{format!("Matched some other path: {}", path)}</p> },
    }
}
