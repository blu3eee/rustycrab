use gloo::utils::window;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let onclick = Callback::from(|_| {
        let window = window();
        window.location().set_href("/").unwrap();
    });

    html! { 
        <header>
            <div class="branding" {onclick}>
                <p>{"Rusty Crab"}</p>
            </div>
            <nav>
                <a href="/about">{"About"}</a>
                // Add more navigation links as needed
            </nav>
        </header>
    }
}
