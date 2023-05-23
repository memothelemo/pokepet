use crate::id::{ApplicationId, ChannelId, GuildId, UserId};
use crate::misc::{ImageHash, Locale};

use serde::{Deserialize, Serialize};

mod ban;
mod boost_level;
mod explicit_content_filter;
mod feature;
mod integration;
mod member;
mod message_notification_level;
mod mfa_level;
mod mutable_feature;
mod nsfw_level;
mod preview;
mod role;
mod system_channel_flags;
mod unavailable;
mod verification_level;
mod widget;

pub use ban::*;
pub use boost_level::*;
pub use explicit_content_filter::*;
pub use feature::*;
pub use integration::*;
pub use member::*;
pub use member::*;
pub use message_notification_level::*;
pub use mfa_level::*;
pub use mutable_feature::*;
pub use nsfw_level::*;
pub use preview::*;
pub use role::*;
pub use system_channel_flags::*;
pub use unavailable::*;
pub use verification_level::*;
pub use widget::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Guild {
  pub id: GuildId,
  pub name: String,
  pub icon: Option<ImageHash>,
  pub splash: Option<ImageHash>,
  pub discovery_splash: Option<ImageHash>,
  pub owner_id: UserId,
  pub afk_channel_id: Option<ChannelId>,
  pub afk_timeout: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub widget_enabled: Option<bool>,
  pub widget_channel_id: Option<ChannelId>,
  pub verification_level: VerificationLevel,
  pub default_message_notifications: MessageNotificationLevel,
  pub explicit_content_filter: ExplicitContentFilter,
  pub roles: Vec<Role>,
  // TODO: emojis
  pub features: Vec<GuildFeature>,
  pub mfa_level: MFALevel,
  pub application_id: Option<ApplicationId>,
  pub system_channel_id: Option<ChannelId>,
  pub system_channel_flags: SystemChannelFlags,
  pub rules_channel_id: Option<ChannelId>,
  pub max_presences: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_members: Option<u64>,
  pub vanity_url_code: Option<String>,
  pub description: Option<String>,
  pub banner: Option<ImageHash>,
  #[serde(rename = "premium_tier")]
  pub boost_level: BoostLevel,
  #[serde(rename = "premium_subscription_count")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boost_count: Option<u64>,
  #[serde(default)]
  pub preferred_locale: Locale,
  pub public_updates_channel_id: Option<ChannelId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_video_channel_users: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_stage_video_channel_users: Option<u64>,
  // TODO: welcome_screen
  pub nsfw_level: GuildNSFWLevel,
  //#[serde(skip_serializing_if = "Option::is_none")]
  //TODO: pub stickers: Option<Vec<Sticker>>,
  #[serde(rename = "premium_progress_bar_enabled")]
  pub boost_progress_bar_enabled: bool,
  pub safety_alerts_channel_id: Option<ChannelId>,
}
