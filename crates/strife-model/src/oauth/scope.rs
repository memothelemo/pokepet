use crate::internal::macros::enum_string;

enum_string! {
  pub enum OAuthScope {
    ActivitiesRead = "activities.read",
    ActivitiesWrite = "activities.write",
    ApplicationBuildsRead = "applications.builds.read",
    ApplicationBuildsUpload = "applications.builds.upload",
    ApplicationCommands = "applications.commands",
    ApplicationCommandsUpdate = "applications.commands.update",
    ApplicationCommandsPermUpdate = "applications.commands.permissions.update",
    ApplicationEntitlements = "applications.entitlements",
    ApplicationStoreUpdate = "applications.store.update",
    Bot = "bot",
    UserConnections = "connections",
    DMChannelsRead = "dm_channels.read",
    Email = "email",
    GroupDMJoin = "gdm.join",
    ShowCurrentUserGuilds = "guilds",
    GuildsJoin = "guilds.join",
    ReadGuildMemberInfo = "guilds.members.read",
    Identify = "identify",
    MessagesRead = "messages.read",
    UserRelationshipsRead = "relationships.read",
    UserRoleConnectionsWrite = "role_connections.write",
    RPC = "rpc",
    RPCActivitiesWrite = "rpc.activities.write",
    RPCNotificationsRead = "rpc.notifications.read",
    RPCVoiceRead = "rpc.voice.read",
    RPCVoiceWrite = "rpc.voice.write",
    Voice = "voice",
    WebhookIncoming = "webhook.incoming",
  }
}
