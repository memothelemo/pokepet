use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

fn main() {
  dotenv().ok();

  let conn =
    PgConnection::establish(&std::env::var("DATABASE_URL").expect("missing database url")).unwrap();

  println!("Hello world!");
}
