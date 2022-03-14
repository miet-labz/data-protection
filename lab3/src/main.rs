use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

mod types;
use types::{TagBITMAPFILEHEADER, TagBITMAPINFOHEADER, TagRGBQUAD, BYTE};

mod input;
use input::*;

mod crypto;
use crypto::*;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

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
            for i in 0..file_content.len() {
                // make pixel from bytes in
            }
        }
        2 => {
            print!("dehide!");
            println!("hello steganography!")
        }
        _ => println!("Err"),
    }

    return Ok(());
}
