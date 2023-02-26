use axum::{routing::get, Router, Server};
use load_dotenv::load_dotenv;

load_dotenv!();

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_request));

    let addr = env!("ADDRESS").parse().unwrap();

    println!("Server running at: http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_request() -> &'static str {
    "Hello World!"
}
