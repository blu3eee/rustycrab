use axum::{ http::Request, middleware::Next, response::IntoResponse };

pub async fn log_route<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse where B: Send {
    println!("Accessing route: {}", req.uri().path());
    next.run(req).await
}
