use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

mod crypto;
use crypto::*;

fn main() -> io::Result<()> {
    let mut out_file = File::create("src/a.txt")?;
    let mut my_str = encrypt_key();
    out_file.write_all(&mut my_str)?;
    my_str.clear();
    //
    println!("Enter source file");
    let source_path = input();

    println!("Enter destination file");
    let dest_path = input();

    println!("Enter key file");
    let key_path = input();
    let encr;
    loop {
        println!("Encrypt or decrypt? (1 - encrypt; 2 - decrypt)");
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim() {
            "1" => {
                encr = true;
                break;
            }
            "2" => {
                encr = false;
                break;
            }
            _ => continue,
        }
    }

    let mut s = Vec::new();
    let mut key = HashMap::new();
    let mut key_buf = Vec::new();
    let mut s_file = File::open(&source_path[0..source_path.len() - 1])?;
    s_file.read_to_end(&mut s)?;
    println!("{:?}", &s);

    let mut key_file = File::open(&key_path[0..key_path.len() - 1])?;
    key_file.read_to_end(&mut key_buf)?;

    for i in (0..key_buf.len()).step_by(2) {
        key.insert(key_buf[i], key_buf[i + 1]);
    }
    if encr {
        encrypt(&mut s, &key);
    } else {
        let dec_key = decrypt_key(&key);
        decrypt(&mut s, &dec_key);
    }
    println!("{:?}", &s);

    let mut out_file = File::create(&dest_path[0..dest_path.len() - 1])?;
    out_file.write_all(&mut s)?;

    s.clear();
    key.clear();
    key_buf.clear();

    Ok(())
}

pub fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    return s;
}
