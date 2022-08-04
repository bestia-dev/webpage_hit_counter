//! postgres_mod.rs

pub async fn select_count() -> Result<i32, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=admin dbname=webpage_hit_counter ",
        tokio_postgres::NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    actix_web::rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT count from hit_counter", &[]).await?;

    // And then check that we got back the same string we sent over.
    let value: i32 = rows[0].get(0);

    Ok(value)
}
