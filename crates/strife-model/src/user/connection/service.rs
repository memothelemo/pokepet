use crate::internal::macros::enum_string;

enum_string! {
  pub enum ConnectionService {
    BattleNet = "battlenet",
    Ebay = "ebay",
    EpicGames = "epicgames",
    Facebook = "facebook",
    GitHub = "github",
    Instagram = "instagram",
    LeagueOfLegends = "leagueoflegends",
    Paypal = "paypal",
    PlayStationNetwork = "playstation",
    Reddit = "reddit",
    RiotGames = "riotgames",
    Spotify = "spotify",
    Skype = "skype",
    Steam = "steam",
    TikTok = "tiktok",
    Twitch = "twitch",
    Twitter = "twitter",
    Xbox = "xbox",
    YouTube = "youtube",
  }
}

#[cfg(test)]
mod service {
  use super::ConnectionService;
  use serde_test::{assert_tokens, Token};

  #[test]
  fn serde() {
    assert_tokens(
      &ConnectionService::PlayStationNetwork,
      &[Token::Str("playstation")],
    );
    assert_tokens(
      &ConnectionService::Unknown("discord".to_string()),
      &[Token::Str("discord")],
    );
  }
}
