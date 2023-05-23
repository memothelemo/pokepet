// use strife_model::presence::Activity;

use strife_model::gateway::GatewayEvent;

fn main() {
  let data = serde_json::from_str::<GatewayEvent>(include_str!("test.json")).unwrap();

  println!("{data:#?}");
}
