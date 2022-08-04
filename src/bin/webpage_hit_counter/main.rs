//! webpage_hit_counter/src/bin/webpage_hit_counter/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

use actix_web::{web, App, HttpServer};
use webpage_hit_counter::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Actix web server started!");

    let pool = deadpool_postgres_start().await;

    let http_server_result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/get_image", web::get().to(get_image))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    println!("");
    println!("Actix web server stopped!");
    // return
    http_server_result
}
