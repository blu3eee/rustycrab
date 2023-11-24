use axum::{ http::Request, middleware::Next, response::IntoResponse };

pub async fn log_route<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse where B: Send {
    println!("Accessing route: {} {}", req.method().as_str().to_uppercase(), req.uri().path());
    next.run(req).await
}
