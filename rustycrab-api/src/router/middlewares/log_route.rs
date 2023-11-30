use axum::{ response::Response, middleware::Next, extract::Request };

pub async fn log_route(req: Request, next: Next) -> Response {
    println!("Accessing route: {} {}", req.method().as_str().to_uppercase(), req.uri().path());
    next.run(req).await
}
