use app::App;

pub mod router;
pub mod app;
pub mod components;
pub mod pages;
pub mod utils;

fn main() {
    yew::Renderer::<App>::new().render();
}
