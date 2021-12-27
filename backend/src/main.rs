use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{services::ServeDir, trace::TraceLayer};

struct Context {
    hello_count: usize,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt::init();
    let main_page = Html(
        tokio::fs::read_to_string("./static/index.html")
            .await
            .unwrap(),
    );

    let ctx = Arc::new(RwLock::new(Context { hello_count: 0 }));
    let app = Router::new()
        .nest(
            "/static",
            get_service(ServeDir::new("./static")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .layer(TraceLayer::new_for_http())
        .route("/", get(move || async move { main_page }));
    let ctx_ = Arc::clone(&ctx);
    let app = app.route("/hello", get(move || get_hello(ctx_)));
    let ctx_ = Arc::clone(&ctx);
    let app = app.route(
        "/hello",
        post(move |a1: Json<SayHello>| say_hello(ctx_, a1)),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_hello(ctx: Arc<RwLock<Context>>) -> impl IntoResponse {
    (StatusCode::OK, Json(ctx.read().await.hello_count))
}

async fn say_hello(ctx: Arc<RwLock<Context>>, Json(payload): Json<SayHello>) -> impl IntoResponse {
    ctx.write().await.hello_count += payload.count;
    (StatusCode::OK, Json(()))
}

#[derive(Deserialize)]
struct SayHello {
    count: usize,
}
