pub fn encrypt(text: String, key: Vec<usize>) -> String {
  let (key_len, text_len) = (key.len(), text.len());
  let modulo: usize = text_len % key_len;
  // copy
  let mut cvec: Vec<char> = text.chars().collect();
  if modulo != 0 {
    for _ in 0..(key_len - modulo) {
      cvec.push('z');
    }
  }
  let text_len = cvec.len();

  let tmp: Vec<char> = cvec.to_vec();
  for i in 0..text_len {
    let idx = i / key_len * key_len + key[i % key_len];
    cvec[i] = tmp[idx];
  }
  return cvec.into_iter().collect();
}

pub fn decrypt(text: String, key: Vec<usize>) -> String {
  let (key_len, text_len) = (key.len(), text.len());
  let mut cvec: Vec<char> = text.chars().collect();
  // copy
  let tmp: Vec<char> = cvec.to_vec();

  for i in 0..text_len {
    let idx = i / key_len * key_len + key[i % key_len];
    cvec[idx] = tmp[i];
  }
  return cvec.into_iter().collect();
}
