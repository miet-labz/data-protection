pub fn encrypt(content: Vec<u8>, key: Vec<usize>) -> Vec<u8> {
  let (key_len, text_len) = (key.len(), content.len());
  let modulo: usize = text_len % key_len;
  // copy
  let mut cvec: Vec<u8> = content.to_vec();
  if modulo != 0 {
    for _ in 0..(key_len - modulo) {
      cvec.push('z' as u8);
    }
  }
  let text_len = cvec.len();

  let tmp: Vec<u8> = cvec.to_vec();
  for i in 0..text_len {
    let idx = i / key_len * key_len + key[i % key_len];
    cvec[i] = tmp[idx];
  }
  return cvec.to_owned();
}

pub fn decrypt(content: Vec<u8>, key: Vec<usize>) -> Vec<u8> {
  let (key_len, text_len) = (key.len(), content.len());
  let mut cvec: Vec<u8> = content.to_vec();
  // copy
  let tmp: Vec<u8> = cvec.to_vec();

  for i in 0..text_len {
    let idx = i / key_len * key_len + key[i % key_len];
    cvec[idx] = tmp[i];
  }
  return cvec.to_owned();
}
