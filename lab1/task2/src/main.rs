use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;

fn decrypt_key(key: &HashMap<u8, u8>) -> HashMap<u8, u8> {
    let mut dec_key = HashMap::new();
    for (k, value) in &*key {
        dec_key.insert(*value, *k);
    }

    dec_key
}

fn encrypt_key() -> Vec<u8>{
    let mut key = Vec::new();
    for i in 0u8..=255{
        key.push(i);
        key.push(255 - i);
    };

    key
}

fn encrypt(source:&mut Vec<u8>, key:& HashMap<u8, u8>){
   for i in 0..source.len(){
       source[i] = key[&source[i]]
   }
}
fn decrypt(source:&mut Vec<u8>, key:& HashMap<u8, u8>){
    let dec_key = decrypt_key(&key);
    for i in 0..source.len(){
        source[i] = dec_key[&source[i]]
    }
}
fn main() -> io::Result<()>{
    // let mut out_file = File::create("/home/simplepc/Documents/Git/MietSecurityLabs/task2/src/key.txt")?;
    // let mut s = encrypt_key();
    // out_file.write_all(&mut s)?;
    // s.clear();
    
    println!("Enter source file");
    let mut source_path = String::new();
    io::stdin().read_line(&mut source_path).expect("Failed to read line");

    println!("Enter destination file");
    let mut dest_path = String::new();
    io::stdin().read_line(&mut dest_path).expect("Failed to read line");

    println!("Enter key file");
    let mut key_path = String::new();
    io::stdin().read_line(&mut key_path).expect("Failed to read line");
    let encr;
    loop {
        println!("Encrypt or decrypt? (1 - encrypt; 2 - decrypt");
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        if s == "1\n" {
            encr = true;
            break;
        } else if s == "2\n" {
            encr = false;
            break;
        }
      }


    let mut s = Vec::new();
    let mut key = HashMap::new();
    let mut key_buf = Vec::new();
    let mut s_file = File::open(&source_path[0..source_path.len()-1])?;
    s_file.read_to_end(&mut s)?;
    println!("{:?}", &s);

    let mut key_file = File::open(&key_path[0..key_path.len()-1])?;
    key_file.read_to_end(&mut key_buf)?;

    for i in (0..key_buf.len()).step_by(2){
        key.insert(key_buf[i], key_buf[i+1]);
    };
    if encr{
        encrypt(&mut s, &key);
    }else{
        let dec_key = decrypt_key(&key);
        decrypt(&mut s, &dec_key);
    }
    
    println!("{:?}", &s);

    let mut out_file = File::create(&dest_path[0..dest_path.len()-1])?;
    out_file.write_all(&mut s)?;

    s.clear();
    key.clear();
    key_buf.clear();


    Ok(())
}
