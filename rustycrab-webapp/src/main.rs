use app::App;

pub mod router;
pub mod app;
pub mod components;
pub mod pages;

fn main() {
    yew::Renderer::<App>::new().render();
}
