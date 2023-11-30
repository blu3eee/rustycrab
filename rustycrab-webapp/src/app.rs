use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{ switch_main, MainRoute };
use crate::components::header::Header;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header/>
            <div class="main_container">
                <BrowserRouter>
                    <Switch<MainRoute> render={switch_main} />
                </BrowserRouter>
            </div>
        </>
    }
}
