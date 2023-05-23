use strife_model::gateway::GatewayEvent;

fn main() {
  let event = serde_json::from_str::<GatewayEvent>(include_str!("test.json")).unwrap();
  println!("{event:#?}");
}
