use crate::internal::macros::enum_string;

enum_string! {
  #[allow(non_camel_case_types)]
  pub enum MutableGuildFeature {
    /// Enables Community Features in the guild
    COMMUNITY = "COMMUNITY",
    /// Enables discovery in the guild, making it publicly listed
    DISCOVERABLE = "DISCOVERABLE",
    /// Pauses all invites/access to the server
    INVITES_DISABLED = "INVITES_DISABLED",
    /// Disables alerts for join raids
    RAID_ALERTS_DISABLED = "RAID_ALERTS_DISABLED",
  }
}
