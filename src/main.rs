use axum::{
    routing::get,
    Router,
    extract::{Path,State},

};
use std::net::SocketAddr;
use std::sync::Arc;
use spdlog::info;
use spdlog::Logger;

#[tokio::main]
async fn main() {

    let default_logger: Arc<Logger> = spdlog::default_logger();
    info!(logger: default_logger, "hello, world");
    
//参考https://github.com/tokio-rs/axum/blob/main/examples/error-handling-and-dependency-injection/src/main.rs
    let app = Router::new().route("/:id", get(handler).with_state(default_logger)); // http://127.0.0.1:3000

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



// 处理器
// async fn handler(default_logger : &Arc<Logger>) -> &'static str {
//     info!(logger: default_logger, "hello, world");
//     "Hello, World!"
// }



async fn handler(Path(id): Path<i32>, State(default_logger): State<Arc<Logger>>) -> String {

     info!(logger: default_logger, "hello, world");

    format!("user id:{}", id)

}

