use dotenvy::dotenv;
use error_stack::{IntoReport, Result, ResultExt};
use figment::Figment;
use secrecy::ExposeSecret;

mod error;
mod impls;
mod structs;

#[cfg(test)]
mod tests;

pub use error::*;
pub use structs::*;

// Other aliases for 'POKEPET_BOT_TOKEN' so that some Discord
// bot hosting providers will be happy and also for lazy people.
static BOT_TOKEN_ALIASES: &[&str] = &["BOT_TOKEN", "DISCORD_TOKEN", "TOKEN"];

const DEFAULT_CONFIG_FILE: &str = "config/config.toml";

impl Settings {
  /// Builds the figment structure to deserialize [`Settings`] object.
  pub fn figment() -> Figment {
    use figment::providers::{Env, Format, Toml};
    use figment::value::UncasedStr;

    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    // Idk how to implement that without relying on hash maps
    static ENV_REPLACEMENTS: Lazy<HashMap<&'static str, &UncasedStr>> = Lazy::new(|| {
      let mut map = HashMap::new();
      map.insert("DATABASE.POOL.SIZE", "database.pool_size".into());
      map.insert(
        "DATABASE.POOL.TIMEOUT.SECS",
        "database.pool_timeout_secs".into(),
      );
      map
    });

    // Figment allows you to use one of many methods handle configurations
    // and combine them at once. I don't have to implement deserialization
    // for both environment variables and config file.
    let figment = Figment::new()
      .merge(Toml::file(DEFAULT_CONFIG_FILE))
      // Aliases, read docs from Settings::init
      .merge(
        Env::raw()
          .only(&["DATABASE_URL"])
          .map(|_| "database.uri".into()),
      )
      .merge(Env::prefixed("POKEPET_").split("_").map(|v| {
        if let Some(alias) = ENV_REPLACEMENTS.get(v.as_str()) {
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

impl Settings {
  /// Reads config from either and merges with:
  /// - `config/config.toml` file (optional)
  /// - Environment variables starting with `POKEPET_`
  /// - Alternative environment variable for `POKEPET_DATABASE_URI`
  ///   - `DATABASE_URL` (testing things with the database)
  ///   - `DATABASE_TEST_URL` (applicable for testing)
  /// - Alternative environment variables for `POKEPET_BOT_TOKEN`
  ///   - `DISCORD_TOKEN`
  ///   - `BOT_TOKEN`
  ///   - `TOKEN`
  pub fn init() -> Result<Self> {
    // Loads `.env` file
    // TODO: Remove this guy here
    dotenv().ok();

    let settings: Self = Self::figment()
      .extract()
      .into_report()
      .change_context(SettingsParseError)?;

    if settings.bot.token.expose_secret() == "unset" {
      return Err(SettingsParseError)
        .into_report()
        .attach_printable("bot token is unset");
    }

    Ok(settings)
  }

  /// Generates TOML documentation for [settings object].
  ///
  /// [settings object]: Settings
  pub fn generate_docs() -> String {
    doku::to_toml_fmt::<Self>(&Default::default())
  }
}

impl DatabaseConfig {
  /// Gets the database url that returns a [secret string].
  ///
  /// [secret string]: SecretString
  pub fn database_url(&self) -> String {
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

    match &self.connect_info {
      DatabaseConnection::Uri(uri) => uri.expose_secret().to_string(),
      DatabaseConnection::Parts(parts) => {
        format!(
          "postgres://{}:{}@{}:{}/{}",
          utf8_percent_encode(&parts.user, NON_ALPHANUMERIC),
          utf8_percent_encode(parts.password.expose_secret(), NON_ALPHANUMERIC),
          parts.host,
          parts.port,
          utf8_percent_encode(&parts.database, NON_ALPHANUMERIC),
        )
      }
    }
  }
}
