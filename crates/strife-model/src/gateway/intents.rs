use crate::internal::macros::bitflags;

bitflags! {
  /// Gateway Intents is used to filter events that the application
  /// should except to recieve and do some action with it.
  ///
  /// This model exists to save some bandwidth and CPU processing
  /// to the client that actually runs the application for the bot.
  pub struct GatewayIntents: u64 {
    const GUILDS = 1 << 0;
    const GUILD_MEMBERS = 1 << 1;
    const GUILD_MODERATION = 1 << 2;
    const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
    const GUILD_INTEGRATIONS = 1 << 4;
    const GUILD_WEBHOOKS = 1 << 5;
    const GUILD_INVITES = 1 << 6;
    const GUILD_VOICE_STATES = 1 << 7;
    const GUILD_PRESENCES = 1 << 8;
    const GUILD_MESSAGES = 1 << 9;
    const GUILD_MESSAGE_REACTIONS = 1 << 10;
    const GUILD_MESSAGE_TYPING = 1 << 11;
    const DIRECT_MESSAGES = 1 << 12;
    const DIRECT_MESSAGE_REACTIONS = 1 << 13;
    const DIRECT_MESSAGE_TYPING = 1 << 14;
    const MESSAGE_CONTENT = 1 << 15;
    const GUILD_SCHEDULED_EVENTS = 1 << 16;
    const AUTO_MODERATION_CONFIGURATION = 1 << 20;
    const AUTO_MODERATION_EXECUTION = 1 << 21;
  }
}

impl GatewayIntents {
  pub const PRIVILEGED: Self = Self::from_bits_truncate(
    Self::GUILD_PRESENCES.bits() | Self::GUILD_MEMBERS.bits() | Self::MESSAGE_CONTENT.bits(),
  );

  pub const fn is_privileged(&self) -> bool {
    self.intersects(Self::PRIVILEGED)
  }
}
