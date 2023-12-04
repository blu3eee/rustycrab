use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{ home::Home, about::About };

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
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
        MainRoute::About => html! {
                <About />
            },
        MainRoute::Post { id } => html! { <p>{format!("You are looking at Post {}", id)}</p> },
        MainRoute::Misc { path } => html! { <p>{format!("Matched some other path: {}", path)}</p> },
    }
}
