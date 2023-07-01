use pokepet_shared::settings::DatabaseConfig;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct DbClient {
  pool: PgPool,
}

impl DbClient {
  #[tracing::instrument(name = "db.connect")]
  pub async fn connect(cfg: &DatabaseConfig) {
    let db_url = cfg.database_url();
    let mut options = PgConnectOptions::from_str(&db_url).unwrap();
    if let Some(enforce_tls) = cfg.enforce_tls {
      let ssl_mode = if enforce_tls {
        PgSslMode::Prefer
      } else {
        PgSslMode::Allow
      };
      options = options.ssl_mode(ssl_mode);
    }

    tracing::info!("Connecting to the database...");

    let now = Instant::now();
    let _pool = PgPoolOptions::new()
      .max_connections(cfg.pool_size.get())
      .acquire_timeout(Duration::from_secs(cfg.pool_timeout_secs.get()))
      .connect_with(options)
      .await
      .unwrap();

    let elapsed = now.elapsed();
    tracing::info!(elapsed = ?elapsed, "Connected to the database");
  }
}
