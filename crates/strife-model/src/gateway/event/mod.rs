use super::{Event, GatewayOpcode};
use crate::gateway::payload::recv::{Hello, InvalidSession};

mod serde_impl;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GatewayEvent {
  Dispatch(u64, Box<Event>),
  Hello(Hello),
  InvalidSession(InvalidSession),
  Reconnect,
  Heartbeat,
  HeartbeatAck,
}

impl GatewayEvent {
  pub const fn opcode(&self) -> GatewayOpcode {
    match self {
      Self::Dispatch(..) => GatewayOpcode::Dispatch,
      Self::Hello(..) => GatewayOpcode::Hello,
      Self::InvalidSession(..) => GatewayOpcode::InvalidSession,
      Self::Reconnect => GatewayOpcode::Reconnect,
      Self::Heartbeat => GatewayOpcode::Heartbeat,
      Self::HeartbeatAck => GatewayOpcode::HeartbeatAck,
    }
  }
}

impl std::fmt::Debug for GatewayEvent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Dispatch(s, e) => {
        write!(f, "Dispatch({s}, ")?;
        e.fmt(f)?;
        write!(f, ")")
      }
      Self::Hello(n) => n.fmt(f),
      Self::InvalidSession(n) => n.fmt(f),
      Self::Reconnect => f.write_str("Reconnect"),
      Self::Heartbeat => f.write_str("Heartbeat"),
      Self::HeartbeatAck => f.write_str("HeartbeatAck"),
    }
  }
}
