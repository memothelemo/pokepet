use super::{ImageHash, BITS_IN_HALF_BYTE, HASH_LEN};

#[must_use]
pub struct ImageHashNibbles {
  index: usize,
  inner: ImageHash,
}

impl ImageHashNibbles {
  pub const fn new(hash: ImageHash) -> Self {
    Self {
      index: 0,
      inner: hash,
    }
  }
}

impl From<ImageHash> for ImageHashNibbles {
  fn from(value: ImageHash) -> Self {
    Self::new(value)
  }
}

impl ImageHashNibbles {
  #[inline]
  const fn get_nibble(&self, nth: usize) -> Option<u8> {
    if nth >= HASH_LEN {
      return None;
    }

    let use_left = nth % 2 == 0;
    let value = self.inner.bytes[nth / 2];

    const RIGHT_MASK: u8 = (1 << BITS_IN_HALF_BYTE) - 1;
    let value = if use_left {
      value >> BITS_IN_HALF_BYTE
    } else {
      value & RIGHT_MASK
    };

    if value >= 10 {
      Some((b'a' - 10) + value)
    } else {
      Some(b'0' + value)
    }
  }
}

impl ExactSizeIterator for ImageHashNibbles {
  fn len(&self) -> usize {
    HASH_LEN
  }
}

impl Iterator for ImageHashNibbles {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.get_nibble(self.index);
    if result.is_some() {
      self.index += 1;
    }
    result
  }

  fn nth(&mut self, n: usize) -> Option<Self::Item> {
    if n >= HASH_LEN {
      self.index = HASH_LEN;
      return None;
    }
    self.index = n;
    self.get_nibble(n)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (HASH_LEN, Some(HASH_LEN))
  }
}
