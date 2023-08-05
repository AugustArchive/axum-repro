#![allow(dead_code)]

use std::net::SocketAddr;
use axum::{extract::State, http::{Request, StatusCode}, middleware::Next, response::{Response, IntoResponse}, Router, routing};

#[derive(Clone)]
struct MyState {
    thing: String,
}

struct DummyResponse {}
impl IntoResponse for DummyResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "dummy response").into_response()
    }
}

async fn dummy<B>(State(state): State<MyState>, req: Request<B>, next: Next<B>) -> Result<Response, impl IntoResponse> {
    dbg!(state.thing);
    Ok::<Response, DummyResponse>(next.run(req).await)
}

#[tokio::main]
async fn main() {
    let state = MyState { thing: "a thing that can be used in the dummy middleware".into() };
    let router = Router::new()
        .route("/", routing::get(|| async { "Hello, world!" }))
        .with_state(state.clone());
        // uncomment the line below for the error
        //.layer(dummy);

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}
