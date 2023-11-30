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
                // <img src="https://media.discordapp.net/attachments/1179886475345657937/1179886642123772015/351356f64ece25ca0433d561c3bc2522_1.png?ex=657b6a10&is=6568f510&hm=25fb33093361b50a8dd9756c51482bb7bc21c71fccc31de20e5cd3fe329bc2dc&=&format=webp&quality=lossless&width=254&height=254" alt="logo"/>
                <p>{"Rusty Crab"}</p>
            </div>
            <nav>
                <a href="/about">{"About"}</a>
                // Add more navigation links as needed
            </nav>
        </header>
    }
}
