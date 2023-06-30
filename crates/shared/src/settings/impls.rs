use figment::providers::{Env, Format, Toml};
use figment::value::UncasedStr;
use figment::Figment;

use error_stack::{IntoReport, Result, ResultExt};
use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::{Settings, SettingsError};

const DEFAULT_CONFIG_FILE: &str = "config/config.toml";

// It converts commonly mistaked variables given from Figment
// into its correct ones because I'm too lazy to modify to handle
// deserialization and error handling each of every environment variables.
static PREFIX_ENV_ALIASES: Lazy<HashMap<&'static str, &UncasedStr>> = Lazy::new(|| {
  let mut map = HashMap::new();
  map.insert("DATABASE.POOL.SIZE", "database.pool_size".into());
  map.insert(
    "DATABASE.POOL.TIMEOUT.SECS",
    "database.pool_timeout_secs".into(),
  );
  map
});

// Other aliases for 'POKEPET_BOT_TOKEN' so that Discord
// bot hosting providers will be happy and also for lazy people.
static BOT_TOKEN_ALIASES: &[&str] = &["BOT_TOKEN", "DISCORD_TOKEN", "TOKEN"];

impl Settings {
  pub fn init() -> Result<Self> {
    dotenvy::dotenv().ok();

    let config = Self::figment()
      .extract::<Self>()
      .into_report()
      .change_context(SettingsError::Parse)?;

    if config.database.uri == "unset" {
      return Err(SettingsError::NotSet("Database uri")).into_report();
    }

    if config.bot.token == "unset" {
      return Err(SettingsError::NotSet("Bot token")).into_report();
    }

    Ok(config)
  }

  /// Generates TOML documentation for [settings object].
  ///
  /// [settings object]: Settings
  pub fn generate_docs() -> String {
    doku::to_toml_fmt::<Self>(&Default::default())
  }
}

impl Settings {
  /// Builds the figment structure for Settings object.
  fn figment() -> Figment {
    // Figment allows you to use one of many methods handle configurations
    // and combine them at once. I don't have to implement deserialization
    // for both environment variables and config file.
    let figment = Figment::new()
      .merge(Toml::file(DEFAULT_CONFIG_FILE))
      // 'DATABASE_URL' is an alias for 'POKEPET_DATABASE_URI'
      .merge(
        Env::raw()
          .only(&["DATABASE_URL"])
          .map(|_| "database.uri".into()),
      )
      .merge(Env::prefixed("POKEPET_").split("_").map(|v| {
        if let Some(alias) = PREFIX_ENV_ALIASES.get(v.as_str()) {
          alias
        } else {
          v
        }
        .into()
      }));

    // I don't know why it works that way
    BOT_TOKEN_ALIASES.iter().fold(figment, |f, alias| {
      f.merge(Env::raw().only(&[alias]).map(|_| "bot.token".into()))
    })
  }
}

#[cfg(test)]
mod tests {
  use super::Settings;
  use super::BOT_TOKEN_ALIASES;

  use figment::Jail;

  fn generate_bot_token() -> String {
    // this is to fix the error
    let mut random_value = rand::random::<u64>().to_string();
    random_value.push('c');
    random_value
  }

  #[test]
  fn prefix_env_aliases() {
    Jail::expect_with(|jail| {
      const POOL_TIMEOUT_SECS: u64 = 10;
      const POOL_SIZE: u64 = 30;

      let generated_token = generate_bot_token();
      jail.set_env("POKEPET_DATABASE_POOL_TIMEOUT_SECS", POOL_TIMEOUT_SECS);
      jail.set_env("POKEPET_DATABASE_POOL_SIZE", POOL_SIZE);
      jail.set_env("POKEPET_BOT_TOKEN", &generated_token);

      let settings = Settings::figment().extract::<Settings>().unwrap();
      assert_eq!(settings.database.pool_timeout_secs.get(), POOL_TIMEOUT_SECS);
      assert_eq!(settings.database.pool_size.get(), POOL_SIZE);
      assert_eq!(settings.bot.token, generated_token);

      Ok(())
    });
  }

  #[test]
  fn bot_token_aliases() {
    // This to avoid false positives
    Jail::expect_with(|jail| {
      for bot_token_alias in BOT_TOKEN_ALIASES {
        let random_value = generate_bot_token();
        jail.set_env(bot_token_alias, &random_value);

        let settings = Settings::figment().extract::<Settings>().unwrap();
        assert_eq!(settings.bot.token, random_value);

        // reset that thing
        jail.set_env(bot_token_alias, "");
      }
      Ok(())
    });
  }
}
