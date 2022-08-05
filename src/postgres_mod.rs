//! postgres_mod.rs

pub async fn select_count(
    db_pool: actix_web::web::Data<deadpool_postgres::Pool>,
    webpage_id: i32,
) -> Result<i32, tokio_postgres::Error> {
    let client: deadpool_postgres::Client = db_pool.get().await.unwrap();

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query(
            "SELECT count from hit_counter where webpage_id = $1",
            &[&webpage_id],
        )
        .await?;

    if rows.len() == 0 {
        // if there is no match, return zero
        Ok(0)
    } else {
        let value: i32 = rows[0].get(0);
        Ok(value)
    }
}
