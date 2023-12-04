use axum::{ Extension, response::Redirect };
use oauth2::{ basic::BasicClient, CsrfToken, Scope };

pub async fn login(Extension(client): Extension<BasicClient>) -> Redirect {
    let auth_url = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()));

    Redirect::to(&auth_url.url().0.to_string())
}
