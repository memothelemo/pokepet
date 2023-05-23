use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Resume {
  pub token: String,
  pub session_id: String,
  #[serde(rename = "seq")]
  pub last_recv_seq: u64,
}
