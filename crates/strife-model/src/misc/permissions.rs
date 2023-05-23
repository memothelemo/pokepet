use bitflags::bitflags;

bitflags! {
  /// It allows to limit and grant certain abilities
  /// to users in Discord.
  ///
  /// Read more about on how their permission system works:
  /// https://discord.com/developers/docs/topics/permissions
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct Permissions: u64 {
    /// Allows creation of instant invites.
    const CREATE_INSTANT_INVITE = 1 << 0;

    /// Allows to kick members in a guild.
    const KICK_MEMBERS = 1 << 1;

    /// Allows to ban members in a guild.
    const BAN_MEMBERS = 1 << 2;

    /// Allows all permissions and bypasses channel
    /// permission overwrites.
    const ADMINISTRATOR = 1 << 3;

    /// Allows management and editing of channels.
    const MANAGE_CHANNELS = 1 << 4;

    /// Allows management and editing of the guild.
    const MANAGE_GUILD = 1 << 5;

    /// Allows for the addition of reactions
    /// to messages in a guild.
    const ADD_REACTIONS = 1 << 6;

    /// Allows for viewing of audit logs.
    const VIEW_AUDIT_LOG = 1 << 7;

    /// Allows for using priority speaker in a
    /// voice channel.
    const PRIORITY_SPEAKER = 1 << 8;

    /// Allows the user to go live.
    const STREAM = 1 << 9;

    /// Allows guild members to view a channel, which
    /// it includes reading messages in text channels
    /// and joining voice channels.
    const VIEW_CHANNEL = 1 << 10;

    /// Allows for sending messages in a channel and
    /// creating threads in a forum (does not allow sending
    /// messages in threads).
    const SEND_MESSAGES = 1 << 11;

    /// Allows for sending of `/tts` messages.
    const SEND_TTS_MESSAGES = 1 << 12;

    /// Allows for deletion of other users' messages.
    const MANAGE_MESSAGES = 1 << 14;

    /// Links sent by users with this permission will
    /// be auto-embedded.
    const EMBED_LINKS = 1 << 14;

    /// Allows for uploading images and files into
    /// channels and threads.
    const ATTACH_FILES = 1 << 15;

    /// Allows for reading of message histories.
    const READ_MESSAGE_HISTORY = 1 << 16;

    /// Allows for using `@everyone` tag to notify
    /// all users in a channel, and the `@here` tag to
    /// notify all online (prescened) users in a channel.
    const MENTION_EVERYONE = 1 << 17;

    /// Allows users to use custom emojis from other servers
    /// (Users must have Nitro or premium subscription required
    /// in order to use it).
    const USE_EXTERNAL_EMOJIS = 1 << 18;

    /// Allows for viewing guild insights.
    const VIEW_GUILD_INSIGHTS = 1 << 19;

    /// Allows to join a voice channel.
    const CONNECT = 1 << 20;

    /// Allows to speak in a voice channel.
    const SPEAK = 1 << 21;

    /// Allows to mute members in a voice channel.
    const MUTE_MEMBERS = 1 << 22;

    /// Allows to deafen members in a voice channel.
    const DEAFEN_MEMBERS = 1 << 23;

    /// Allows to move members between voice channels.
    const MOVE_MEMBERS = 1 << 24;

    /// Allows for using voice-activity-detection
    /// in a voice channel.
    const USE_VAD = 1 << 25;

    /// Allows to modify of a guild member's nickname.
    const CHANGE_NICKNAME = 1 << 26;

    /// Allows to modify other guild members' nickname
    const MANAGE_NICKNAME = 1 << 27;

    /// Allows management and editing of guild roles.
    const MANAGE_ROLES = 1 << 28;

    /// Allows management and editing of guild webhooks.
    const MANAGE_WEBHOOKS = 1 << 29;

    /// Allows management and editing of guild's emojis and stickers.
    const MANAGE_EMOJIS_AND_STICKERS = 1 << 30;

    /// Allows members to use application members
    /// including slash commands, and context menu commands.
    const USE_APPLICATION_COMMANDS = 1 << 31;

    /// Allows for requesting to peak in stage channels.
    ///
    /// **NOTE**:
    /// According to Discord, this permission is under active development.
    /// It may be changed or removed in the future, if you have issues
    /// with this permission please let me know in GitHub issues.
    const REQUEST_TO_SPEAK = 1 << 32;

    /// Allows for creating, editing and deleting
    /// scheduled events in a guild.
    const MANAGE_EVENTS = 1 << 33;

    /// Allows for deleting and archiving threads, and viewing all
    /// private guild threads
    const MANAGE_THREADS = 1 << 34;

    /// Allows for creating public and announcement threads
    const CREATE_PUBLIC_THREADS = 1 << 35;

    /// Allows for creating private threads
    const CREATE_PRIVATE_THREADS = 1 << 36;

    /// Allows the usage of custom stickers from other servers
    const USE_EXTERNAL_STICKERS = 1 << 37;

    /// Allows for sending messages in threads
    const SEND_MESSAGES_IN_THREADS = 1 << 38;

    /// Allows for using Activities (applications with
    /// the EMBEDDED flag) in a voice channel
    const USE_EMBEDDED_ACTIVITIES = 1 << 39;

    /// Allows for timing out users to prevent them from
    /// sending or reacting to messages in chat and threads,
    /// and from speaking in voice and stage channels
    const MODERATE_MEMBERS = 1 << 40;
  }
}

impl Permissions {
  pub const MFA_REQUIRED: Self = Self::from_bits_truncate(
    Self::KICK_MEMBERS.bits()
      | Self::BAN_MEMBERS.bits()
      | Self::ADMINISTRATOR.bits()
      | Self::MANAGE_CHANNELS.bits()
      | Self::MANAGE_GUILD.bits()
      | Self::MANAGE_MESSAGES.bits()
      | Self::MANAGE_ROLES.bits()
      | Self::MANAGE_WEBHOOKS.bits()
      | Self::MANAGE_EMOJIS_AND_STICKERS.bits()
      | Self::MANAGE_THREADS.bits(),
  );

  pub fn requires_mfa(self) -> bool {
    !(Self::MFA_REQUIRED & self).is_empty()
  }
}

impl Permissions {
  pub const TEXT_CHANNEL: Self = Self::from_bits_truncate(
    Self::CREATE_INSTANT_INVITE.bits()
      | Self::MANAGE_CHANNELS.bits()
      | Self::ADD_REACTIONS.bits()
      | Self::VIEW_CHANNEL.bits()
      | Self::SEND_MESSAGES.bits()
      | Self::SEND_TTS_MESSAGES.bits()
      | Self::MANAGE_MESSAGES.bits()
      | Self::EMBED_LINKS.bits()
      | Self::ATTACH_FILES.bits()
      | Self::READ_MESSAGE_HISTORY.bits()
      | Self::MENTION_EVERYONE.bits()
      | Self::USE_EXTERNAL_EMOJIS.bits()
      | Self::MANAGE_ROLES.bits()
      | Self::MANAGE_WEBHOOKS.bits()
      | Self::USE_APPLICATION_COMMANDS.bits()
      | Self::MANAGE_THREADS.bits()
      | Self::CREATE_PUBLIC_THREADS.bits()
      | Self::CREATE_PRIVATE_THREADS.bits()
      | Self::USE_EXTERNAL_STICKERS.bits()
      | Self::SEND_MESSAGES_IN_THREADS.bits(),
  );

  pub fn has_text_perms(self) -> bool {
    if self.contains(Self::ADMINISTRATOR) {
      true
    } else {
      !(Self::TEXT_CHANNEL & self).is_empty()
    }
  }
}

impl Permissions {
  pub const VOICE_CHANNEL: Self = Self::from_bits_truncate(
    Self::CREATE_INSTANT_INVITE.bits()
      | Self::MANAGE_CHANNELS.bits()
      | Self::ADD_REACTIONS.bits()
      | Self::PRIORITY_SPEAKER.bits()
      | Self::STREAM.bits()
      | Self::VIEW_CHANNEL.bits()
      | Self::MANAGE_MESSAGES.bits()
      | Self::EMBED_LINKS.bits()
      | Self::ATTACH_FILES.bits()
      | Self::READ_MESSAGE_HISTORY.bits()
      | Self::MENTION_EVERYONE.bits()
      | Self::USE_EXTERNAL_EMOJIS.bits()
      | Self::CONNECT.bits()
      | Self::SPEAK.bits()
      | Self::MUTE_MEMBERS.bits()
      | Self::DEAFEN_MEMBERS.bits()
      | Self::MOVE_MEMBERS.bits()
      | Self::USE_VAD.bits()
      | Self::MANAGE_ROLES.bits()
      | Self::USE_APPLICATION_COMMANDS.bits()
      | Self::MANAGE_EVENTS.bits()
      | Self::USE_EXTERNAL_STICKERS.bits()
      | Self::USE_EMBEDDED_ACTIVITIES.bits(),
  );

  pub fn has_voice_perms(self) -> bool {
    if self.contains(Self::ADMINISTRATOR) {
      true
    } else {
      !(Self::VOICE_CHANNEL & self).is_empty()
    }
  }
}

impl Permissions {
  pub const STAGED_CHANNEL: Self = Self::from_bits_truncate(
    Self::CREATE_INSTANT_INVITE.bits()
      | Self::MANAGE_CHANNELS.bits()
      | Self::ADD_REACTIONS.bits()
      | Self::VIEW_CHANNEL.bits()
      | Self::MENTION_EVERYONE.bits()
      | Self::CONNECT.bits()
      | Self::MUTE_MEMBERS.bits()
      | Self::DEAFEN_MEMBERS.bits()
      | Self::MOVE_MEMBERS.bits()
      | Self::MANAGE_ROLES.bits()
      | Self::REQUEST_TO_SPEAK.bits()
      | Self::MANAGE_EVENTS.bits(),
  );

  pub fn has_staged_perms(self) -> bool {
    if self.contains(Self::ADMINISTRATOR) {
      true
    } else {
      !(Self::STAGED_CHANNEL & self).is_empty()
    }
  }
}

impl std::fmt::Display for Permissions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0 .0.fmt(f)
  }
}

// This needs custom deserialization than our internal macro
impl<'de> serde::Deserialize<'de> for Permissions {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = Permissions;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("discord permission flags")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        let value = v.parse::<u64>().map_err(serde::de::Error::custom)?;
        self.visit_u64(value)
      }

      fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        if v < 0 {
          Err(serde::de::Error::custom(
            "attempt to deserialize with negative integer",
          ))
        } else {
          self.visit_u64(v as u64)
        }
      }

      fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Permissions::from_bits_truncate(v))
      }
    }

    deserializer.deserialize_any(Visitor)
  }
}

impl serde::Serialize for Permissions {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_str(&self.bits())
  }
}

#[cfg(test)]
mod tests {
  use super::Permissions;
  use serde_test::{assert_de_tokens, assert_tokens, Token};

  #[test]
  fn serde() {
    let flags = Permissions::ADMINISTRATOR | Permissions::ADD_REACTIONS;

    assert_tokens(&flags, &[Token::Str("72")]);
    assert_de_tokens(&flags, &[Token::U64(72)]);
  }

  #[test]
  fn has_stage_perms() {
    let flags = Permissions::CREATE_INSTANT_INVITE;
    assert!(flags.has_staged_perms());

    let flags = Permissions::empty();
    assert!(!flags.has_staged_perms());

    let flags = Permissions::CREATE_INSTANT_INVITE | Permissions::EMBED_LINKS;
    assert!(flags.has_staged_perms());

    let flags = Permissions::ADMINISTRATOR;
    assert!(flags.has_staged_perms());
  }

  #[test]
  fn has_voice_perms() {
    let flags = Permissions::CREATE_INSTANT_INVITE;
    assert!(flags.has_voice_perms());

    let flags = Permissions::empty();
    assert!(!flags.has_voice_perms());

    let flags = Permissions::CREATE_INSTANT_INVITE | Permissions::REQUEST_TO_SPEAK;
    assert!(flags.has_voice_perms());

    let flags = Permissions::ADMINISTRATOR;
    assert!(flags.has_voice_perms());
  }

  #[test]
  fn has_text_perms() {
    let flags = Permissions::CREATE_INSTANT_INVITE;
    assert!(flags.has_text_perms());

    let flags = Permissions::empty();
    assert!(!flags.has_text_perms());

    let flags = Permissions::CREATE_INSTANT_INVITE | Permissions::SPEAK;
    assert!(flags.has_text_perms());

    let flags = Permissions::ADMINISTRATOR;
    assert!(flags.has_text_perms());
  }

  #[test]
  fn requires_mfa() {
    let flags = Permissions::KICK_MEMBERS | Permissions::SEND_TTS_MESSAGES;
    assert!(flags.requires_mfa());

    let flags = Permissions::MFA_REQUIRED;
    assert!(flags.requires_mfa());

    let flags = Permissions::SEND_TTS_MESSAGES;
    assert!(!flags.requires_mfa());
  }
}
