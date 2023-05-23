use crate::internal::macros::enum_string;

enum_string! {
  #[derive(Default)]
  pub enum Locale {
    Indonesian = "id",
    Danish = "da",
    German = "de",
    EnglishGB = "en-GB",
    #[default]
    EnglishUS = "en-US",
    Spanish = "es-ES",
    French = "fr",
    Croatian = "hr",
    Italian = "it",
    Lithuanian = "lt",
    Hungarian = "hu",
    Dutch = "nl",
    Norwegian = "no",
    Polish = "pl",
    PortugueseBR = "pt-BR",
    Romania = "ro",
    Finnish = "fi",
    Swedish = "sv-SE",
    Vietnamese = "vi",
    Turkish = "tr",
    Czech = "cs",
    Greek = "el",
    Bulgarian = "bg",
    Russian = "ru",
    Ukrainian = "uk",
    Hindi = "hi",
    Thai = "th",
    ChineseCH = "zh-CN",
    Japanese = "ja",
    ChineseTW = "zh-TW",
    Korean = "ko",
  }
}

#[cfg(test)]
mod tests {
  use super::Locale;
  use serde_test::{assert_tokens, Token};

  #[test]
  fn serde() {
    // https://en.wikipedia.org/wiki/Hiligaynon_language
    assert_tokens(
      &Locale::Unknown("hil-PH".to_string()),
      &[Token::Str("hil-PH")],
    );
    assert_tokens(&Locale::EnglishUS, &[Token::Str("en-US")]);
  }
}
