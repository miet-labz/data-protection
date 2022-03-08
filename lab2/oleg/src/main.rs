use rand::Rng;
use std::io;
use std::vec::Vec;

pub fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    return s.trim().to_owned();
}

fn enter_2_numbers() -> (u32, u32) {
    let vec = input()
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("Wrong input!"))
        .collect::<Vec<u32>>();
    (vec[0], vec[1])
}

fn enter_encr_str_to_vec() -> Vec<u32> {
    return input()
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("Wrong input!"))
        .collect::<Vec<u32>>();
}

fn select_option() -> Result<i32, std::num::ParseIntError> {
    println!("1-encrypt, 2-decrypt:");
    let buf = input();
    let opt = buf.parse::<i32>();
    opt
}

fn prime_dividers(n: u32, f_n: u32) -> Vec<u32> {
    let mut sd = Vec::new();
    for i in 2..n {
        if num_integer::gcd(f_n, i) == 1 {
            sd.push(i);
        }
    }
    sd
}

fn random_vec_elem(sd: Vec<u32>) -> u32 {
    let mut rng = rand::thread_rng();
    let id: usize = rng.gen::<usize>() % sd.len();
    sd[id]
}

fn modpow(c: u32, d: u32, n: u32) -> u32 {
    let mut res = 1;
    for _i in 1..=d {
        res = (res * c) % n;
    }
    res
}

fn main() {
    loop {
        match select_option().unwrap() {
            1 => {
                encrypt();
                break;
            }
            2 => {
                decrypt();
                break;
            }
            _ => println!("Try again!"),
        }
    }
}

pub fn encrypt() {
    println!("Enter 2 prime nubers:");
    let (p, q) = enter_2_numbers();
    println!("Enter message:");
    let s = input();
    let n = p * q;
    let euler_n = (p - 1) * (q - 1);
    let e = random_vec_elem(prime_dividers(n, euler_n));
    println!("Public key: ({},{})", e, n);
    let mut k = 0;
    let d: u32;
    loop {
        k += 1;
        if (k * euler_n + 1) % e == 0 {
            d = (k * euler_n + 1) / e;
            break;
        }
    }
    println!("Private key: ({},{})", d, n);
    let mut enc_msg: Vec<u32> = Vec::new();
    for c in s.chars() {
        enc_msg.push(modpow(c as u32, e, n));
    }
    for num in enc_msg {
        print!("{} ", num);
    }
    println!()
}

pub fn decrypt() {
    println!("Enter private key:");
    let (d, n) = enter_2_numbers();
    println!("Enter encrypted message:");
    let encr_vec = enter_encr_str_to_vec();
    let s = encr_vec
        .to_vec()
        .into_iter()
        .map(|c| (modpow(c as u32, d, n) as u8 as char))
        .collect::<String>();
    println!("{}", s);
}
