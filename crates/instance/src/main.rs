use pokepet_db::DbClient;
use pokepet_shared::settings::Settings;

fn main() {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .compact()
        .init();

      let settings = Settings::init().unwrap();
      DbClient::connect(&settings.database).await;
      // println!("{settings}");
    })
}
