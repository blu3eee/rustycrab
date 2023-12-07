use rustycrab_model::auth::SessionUserData;
use yew_hooks::prelude::*;

use yew::{ function_component, Html, html };
use crate::utils::{ api_fetch, Error };

async fn fetch_user_data() -> Result<Option<SessionUserData>, Error> {
    Ok(api_fetch::<Option<SessionUserData>>("/auth/state").await?)
}

#[function_component(About)]
pub fn page() -> Html {
    let user_state = use_async(async move { fetch_user_data().await });

    {
        let _ = user_state.clone();
        use_async_with_options(
            async move {
                fetch_user_data().await
            },
            // This will load data automatically when mount.
            UseAsyncOptions::enable_auto()
        );
    }

    html! {
        <>  
            <p>
                {
                    if user_state.loading {
                        html! { "Loading..." }
                    } else {
                        html! { "About you"}
                    }
                }
            </p>
            {
                user_state.data.as_ref().map_or_else(|| html! {}, |user| if let Some(user) = user { html! {
                    <>
                        <p>{ "Username: " }<b>{ &user.user_id }</b></p>
                        // other user fields...
                    </>
                }} else {
                    html! {
                        <>
                        <p>{"You're not logged in yet"}</p>
                        </>
                    }
                })
            }
            <p>
                {
                    user_state.error.as_ref().map_or_else(|| html! {}, |error| match error {
                        Error::DeserializeError => html! { "DeserializeError" },
                        Error::RequestError => html! { "RequestError" },
                        // handle other errors...
                    })
                }
            </p>
        </>
    }
}
