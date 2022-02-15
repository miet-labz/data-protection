use std::collections::HashMap;

pub fn decrypt_key(key: &HashMap<u8, u8>) -> HashMap<u8, u8> {
  let mut dec_key = HashMap::new();
  for (k, value) in &*key {
    dec_key.insert(*value, *k);
  }
  return dec_key;
}

pub fn encrypt_key() -> Vec<u8> {
  let mut key = Vec::new();
  for i in 0u8..=255 {
    key.push(i);
    key.push(255 - i);
  }

  return key;
}

pub fn encrypt(source: &mut Vec<u8>, key: &HashMap<u8, u8>) {
  for i in 0..source.len() {
    source[i] = key[&source[i]]
  }
}
pub fn decrypt(source: &mut Vec<u8>, key: &HashMap<u8, u8>) {
  let dec_key = decrypt_key(&key);
  for i in 0..source.len() {
    source[i] = dec_key[&source[i]]
  }
}
