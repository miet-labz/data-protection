use std::io;

pub fn input() -> String {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("Failed to read line");
  return s.trim().to_owned();
}

pub fn select_option() -> Result<u8, std::num::ParseIntError> {
  println!("1- hide in image, 2- find in image:");
  let buf = input();
  let opt = buf.parse::<u8>();
  opt
}
