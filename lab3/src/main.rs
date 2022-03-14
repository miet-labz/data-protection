use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

mod types;
use types::{TagRGBQUAD, BYTE};

mod input;
use input::*;

mod crypto;
use crypto::*;

pub fn main() -> io::Result<()> {
    println!("Enter image path");
    let image_path = input();
    println!("Enter file path");
    let file_path = input();
    let mut file_content: Vec<BYTE> = Vec::new();
    let mut file = File::open(&file_path)?;
    file.read_to_end(&mut file_content)?;
    let mut image_content: Vec<BYTE> = Vec::new();
    let mut image = File::open(&image_path)?;
    image.read_to_end(&mut image_content)?;

    match select_option().unwrap() {
        1 => {
            let mut file_out:Vec<BYTE> = Vec::new();
            let mut file_body:Vec<BYTE> = Vec::new();
            for i in 0..54 {
                file_out.push(image_content[i])
            }
            for i in 54..image_content.len() {
                file_body.push(image_content[i])
            }
            let mut pixels = TagRGBQUAD::read(file_body);
            let mut byte_num = 0;
            for byte in file_content{
                hide_byte_into_pixel(&mut pixels[byte_num], byte);
                byte_num += 1;
            }
            for px in pixels{
                file_out.append(&mut TagRGBQUAD::to_bytes(&px))
            }
            let mut res = File::create(&image_path)?;
            res.write_all(&mut file_out)?;
        }
        2 => {
            print!("dehide!");
            let mut decr_mes:Vec<BYTE> = Vec::new();
            let mut file_body:Vec<BYTE> = Vec::new();
            for i in 54..image_content.len() {
                file_body.push(image_content[i])
            }
            let mut byte;
            for px in TagRGBQUAD::read(file_body){
                byte = get_hidden_byte(px);
                if byte != 0xFF{
                decr_mes.push(byte)
                }else{
                    break
                }
            }
            let mut res = File::create(&file_path)?;
            res.write_all(&mut decr_mes)?;
        }
        _ => println!("Err"),
    }

    return Ok(());
}
