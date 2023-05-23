#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color(u32);

const MAX_RGB_VALUE: f64 = 255.;
const HUE_MAX_VALUE: f64 = 360.;

// Constants
impl Color {
  pub const WHITE: Self = Self(0xFFFFFF);
  pub const BLACK: Self = Self(0x000000);

  // Discord default role colors
  //
  // Thanks to (Immotay from reddit):
  // https://www.reddit.com/r/discordapp/comments/849bxc/what_are_the_hex_values_of_all_the_default_role/
  pub const LIGHT_CYAN: Self = Self(0x1ABC9C);
  pub const DARK_CYAN: Self = Self(0x11806A);

  pub const LIGHT_GREEN: Self = Self(0x2ECC71);
  pub const DARK_GREEN: Self = Self(0x1F8B4C);

  pub const LIGHT_BLUE: Self = Self(0x3498DB);
  pub const DARK_BLUE: Self = Self(0x206694);

  pub const LIGHT_PURPLE: Self = Self(0x9B59B6);
  pub const DARK_PURPLE: Self = Self(0x71368A);

  pub const LIGHT_RED: Self = Self(0xe91e63);
  pub const DARK_RED: Self = Self(0xad1457);

  pub const LIGHT_YELLOW: Self = Self(0xf1c40f);
  pub const DARK_YELLOW: Self = Self(0xc27c0e);

  pub const LIGHT_GOLD: Self = Self(0xe67e22);
  pub const DARK_GOLD: Self = Self(0xa84300);

  pub const LIGHT_ORANGE: Self = Self(0xe74c3c);
  pub const DARK_ORANGE: Self = Self(0x992d22);

  pub const LIGHT_GRAY: Self = Self(0x95a5a6);
  pub const DARK_GRAY: Self = Self(0x979c9f);
}

impl Color {
  #[must_use]
  pub const fn code(&self) -> u32 {
    self.0
  }

  #[must_use]
  pub fn hsl(&self) -> (f64, f64, f64) {
    let (r, g, b) = self.rgb();

    let red = r as f64 / MAX_RGB_VALUE;
    let green = g as f64 / MAX_RGB_VALUE;
    let blue = b as f64 / MAX_RGB_VALUE;

    let max = if red > green && red > blue {
      red
    } else if green > blue {
      green
    } else {
      blue
    };

    let min = if red < green && red < blue {
      red
    } else if green < blue {
      green
    } else {
      blue
    };

    let mut hue = 0.;
    let mut saturation = 0.;
    let lightness = (max + min) / 2.;

    if max != min {
      let d = max - min;
      saturation = if lightness > 0.5 {
        d / (2. - max - min)
      } else {
        d / (max + min)
      };

      hue = match max {
        c if c == red => (green - blue) / d + if g < b { 6. } else { 0. },
        c if c == green => (blue - red) / d + 2.,
        c if c == blue => (red - green) / d + 4.,
        _ => unreachable!(),
      } / 6.;
    }

    (hue * HUE_MAX_VALUE, saturation, lightness)
  }

  #[must_use]
  pub const fn rgb(&self) -> (u8, u8, u8) {
    (self.red(), self.green(), self.blue())
  }
}

impl Color {
  #[must_use]
  pub const fn red(&self) -> u8 {
    (self.0 >> 16) as u8
  }

  #[must_use]
  pub const fn green(&self) -> u8 {
    (self.0 >> 8) as u8
  }

  #[must_use]
  pub const fn blue(&self) -> u8 {
    self.0 as u8
  }
}

impl Color {
  #[must_use]
  pub const fn from_code(code: u32) -> Self {
    // Strip off unnecessary bits (8 bits to be exact)
    Self(code & Self::WHITE.0)
  }

  #[must_use]
  pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
    Self(((r as u32) << 16) | ((g as u32) << 8) | b as u32)
  }
}

impl std::fmt::Debug for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Color({}, {}, {})",
      self.red(),
      self.green(),
      self.blue()
    )
  }
}

impl std::fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "0x{:0>6x}", self.0)
  }
}

impl<'de> serde::Deserialize<'de> for Color {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = Color;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "integer color code")
      }

      fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Color::from_code(v as u32))
      }
    }

    deserializer.deserialize_u64(Visitor)
  }
}

impl serde::Serialize for Color {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl PartialEq<u32> for Color {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_test::{assert_tokens, Token};

  const COLORS: &[(u32, u8, u8, u8)] = &[
    (0x27374D, 39, 55, 77),
    (0xDBA93F, 219, 169, 63),
    (0x58B530, 88, 181, 48),
    (0xEA31C5, 234, 49, 197),
  ];

  #[test]
  fn rgb() {
    for (hex, r, g, b) in COLORS {
      let color = Color::from_code(*hex);
      assert_eq!(color.red(), *r);
      assert_eq!(color.green(), *g);
      assert_eq!(color.blue(), *b);
    }
  }

  #[test]
  fn from_rgb() {
    for (hex, r, g, b) in COLORS {
      let color = Color::from_rgb(*r, *g, *b);
      assert_eq!(color.code(), *hex);
    }
  }

  #[test]
  fn serde() {
    for (_, r, g, b) in COLORS {
      let color = Color::from_rgb(*r, *g, *b);
      assert_tokens(&color, &[Token::U32(color.0)]);
    }
  }
}
