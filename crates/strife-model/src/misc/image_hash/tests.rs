use super::ImageHash;
use serde_test::{assert_tokens, Token};

const ANIMATED_HASH: &[u8] = b"a_1269e74af4df7417b13759eae50c83dc";
const HASH: &[u8] = b"3e041ecc0031e259d4350618835c98f7";

const ANIMATED_HASH_BYTES: &[u8] = &[
  0x12, 0x69, 0xe7, 0x4a, 0xf4, 0xdf, 0x74, 0x17, 0xb1, 0x37, 0x59, 0xea, 0xe5, 0x0c, 0x83, 0xdc,
];

const HASH_BYTES: &[u8] = &[
  0x3e, 0x04, 0x1e, 0xcc, 0x00, 0x31, 0xe2, 0x59, 0xd4, 0x35, 0x06, 0x18, 0x83, 0x5c, 0x98, 0xf7,
];

#[test]
fn serde() {
  let animated_hash = ImageHash::parse(ANIMATED_HASH).unwrap();
  assert_tokens(
    &animated_hash,
    &[Token::Str(std::str::from_utf8(ANIMATED_HASH).unwrap())],
  );

  let hash = ImageHash::parse(HASH).unwrap();
  assert_tokens(&hash, &[Token::Str(std::str::from_utf8(HASH).unwrap())]);
}

#[test]
fn display() {
  let hash = ImageHash::parse(ANIMATED_HASH).unwrap();
  let expected = std::str::from_utf8(ANIMATED_HASH).unwrap();
  assert_eq!(hash.to_string(), expected);

  let hash = ImageHash::parse(HASH).unwrap();
  let expected = std::str::from_utf8(HASH).unwrap();
  assert_eq!(hash.to_string(), expected);
}

#[test]
fn parse() {
  let hash = ImageHash::parse(ANIMATED_HASH).unwrap();
  assert_eq!(hash.animated(), true);
  assert_eq!(hash.bytes(), ANIMATED_HASH_BYTES);

  let hash = ImageHash::parse(HASH).unwrap();
  assert_eq!(hash.animated(), false);
  assert_eq!(hash.bytes(), HASH_BYTES);
}

#[test]
fn nibbles() {
  let hash = ImageHash::parse(HASH).unwrap();

  let mut iter = hash.nibbles().enumerate();
  while let Some((i, nibble)) = iter.next() {
    let expected = HASH[i];
    assert_eq!(nibble, expected);
  }

  assert_eq!(iter.next(), None);
}
