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
    println!("Actix web server started on localhost:8011!");
    println!("test it with curl or browser:");
    println!("http://localhost:8011/webpage_hit_counter/get_svg_image/555555.svg");

    let pool = deadpool_postgres_start().await;
    // Check the connection to postgres database and panic if error
    let client: deadpool_postgres::Client = pool.get().await.unwrap();
    drop(client);

    println!("start server");
    let http_server_result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/webpage_hit_counter/get_svg_image/{webpage_id}.svg", web::get().to(get_svg_image))
    })
    .bind(("0.0.0.0", 8011))?
    .run()
    .await;

    println!();
    println!("Actix web server stopped!");
    // return
    http_server_result
}
