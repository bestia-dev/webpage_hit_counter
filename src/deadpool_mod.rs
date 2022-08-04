//! deadpool_mod.rs

pub async fn deadpool_postgres_start() -> deadpool_postgres::Pool {
    // this loads our .env file and includes the values in std::env
    dotenv::dotenv().ok();

    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(std::env::var("PG.HOST").unwrap().as_str());
    pg_config.user(std::env::var("PG.USER").unwrap().as_str());
    pg_config.dbname(std::env::var("PG.DBNAME").unwrap().as_str());
    let mgr_config = deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    };
    let mgr = deadpool_postgres::Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
    let pool = deadpool_postgres::Pool::builder(mgr)
        .max_size(16)
        .build()
        .unwrap();
    // return
    pool
}
