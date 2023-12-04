use yew::{ function_component, Html, html, Callback };
use crate::components::button::Button;
use web_sys::{ window, MouseEvent };

#[function_component(Home)]
pub fn page() -> Html {
    let onclick = Callback::from(move |_: MouseEvent| {
        if let Some(window) = window() {
            window.location().set_href("http://localhost:8080/api/auth/login").unwrap();
        }
    });
    html! {
        <>
            <div>
                {"Ultimate Discord music bot powered by Rust"}
            </div>
            <Button label="Login with discord"  icon="https://www.svgrepo.com/download/353655/discord-icon.svg" onclick={onclick}/>
        </>
    }
}
