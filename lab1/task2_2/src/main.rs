use std::fs;
use std::io;
use std::vec::Vec;

mod crypto;
use crypto::*;

fn main() -> io::Result<()> {
    println!("Enter source file");
    let source_path = input();

    println!("Enter destination file");
    let dest_path = input();

    println!("Enter key file");
    let key_path = input();

    // reading from file
    let src_str = fs::read_to_string(source_path).expect("Something went wrong reading the file");
    let key = fs::read_to_string(key_path).expect("Something went wrong reading the file");
    let key = key
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    println!("{:?}", key);
    let res: String;
    loop {
        println!("Encrypt or decrypt? (1 - encrypt; 2 - decrypt)");
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim() {
            "1" => {
                res = encrypt(src_str, key);
                break;
            }
            "2" => {
                res = decrypt(src_str, key);
                break;
            }
            _ => continue,
        }
    }

    fs::write(dest_path, res).expect("Something went wrong writing the file");
    return Ok(());
}
pub fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    return s.trim().to_owned();
}
