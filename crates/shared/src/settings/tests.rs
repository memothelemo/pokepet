use super::Settings;
use super::BOT_TOKEN_ALIASES;
use figment::Jail;
use secrecy::ExposeSecret;

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
    const POOL_SIZE: u32 = 30;

    let generated_token = generate_bot_token();
    jail.set_env("POKEPET_DATABASE_POOL_TIMEOUT_SECS", POOL_TIMEOUT_SECS);
    jail.set_env("POKEPET_DATABASE_POOL_SIZE", POOL_SIZE);
    jail.set_env("POKEPET_BOT_TOKEN", &generated_token);

    let settings = Settings::figment().extract::<Settings>().unwrap();
    assert_eq!(settings.database.pool_timeout_secs.get(), POOL_TIMEOUT_SECS);
    assert_eq!(settings.database.pool_size.get(), POOL_SIZE);
    assert_eq!(settings.bot.token.expose_secret(), &generated_token);

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
      assert_eq!(settings.bot.token.expose_secret(), &random_value);

      // reset that thing
      jail.set_env(bot_token_alias, "");
    }
    Ok(())
  });
}
