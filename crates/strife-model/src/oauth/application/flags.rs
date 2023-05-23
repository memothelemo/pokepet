use crate::internal::macros::bitflags;

bitflags! {
  pub struct ApplicationFlags: u32 {
    /// Indicates if an app uses the Auto Moderation API
    const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
    /// Intent required for bots in 100 or more servers to receive presence_update events
    const GATEWAY_PRESENCE = 1 << 12;
    /// Intent required for bots in under 100 servers to receive presence_update events, found on the Bot page in your app's settings
    const GATEWAY_PRESENCE_LIMITED = 1 << 13;
    /// Intent required for bots in 100 or more servers to receive member-related events like guild_member_add. See the list of member-related events under GUILD_MEMBERS
    const GATEWAY_GUILD_MEMBERS = 1 << 14;
    /// Intent required for bots in under 100 servers to receive member-related events like guild_member_add, found on the Bot page in your app's settings. See the list of member-related events under GUILD_MEMBERS
    const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
    /// Indicates unusual growth of an app that prevents verification
    const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
    /// Indicates if an app is embedded within the Discord client (currently unavailable publicly)
    const EMBEDDED = 1 << 17;
    /// Intent required for bots in 100 or more servers to receive message content
    const GATEWAY_MESSAGE_CONTENT = 1 << 18;
    /// Intent required for bots in under 100 servers to receive message content, found on the Bot page in your app's settings
    const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
    /// Indicates if an app has registered global application commands
    const APPLICATION_COMMAND_BADGE = 1 << 23;
  }
}
