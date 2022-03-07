use rand::Rng;
use std::io;
use std::vec::Vec;
extern crate num_integer;

fn enter_p_q() -> (u32, u32) {
    let mut buf = String::new();
    println!("Enter 2 simple nubers:");
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.remove(buf.len() - 1);
    let vec = buf
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("Wrong input!"))
        .collect::<Vec<u32>>();
    (vec[0], vec[1])
}

fn enter_d_n() -> (u32, u32) {
    let mut buf = String::new();
    println!("Enter private key:");
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.remove(buf.len() - 1);
    let vec = buf
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("Wrong input!"))
        .collect::<Vec<u32>>();
    (vec[0], vec[1])
}

fn enter_str() -> String {
    println!("Enter message:");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.remove(buf.len() - 1);
    buf
}
fn enter_encr_str() -> Vec<u32> {
    println!("Enter encrypted message:");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    let out = buf
        .split_whitespace()
        .map(|n| n.parse::<u32>().expect("Wrong input!"))
        .collect::<Vec<u32>>();
    out
}

fn option() -> io::Result<bool> {
    let mut buf = String::new();
    println!("1-encrypt, 2-decrypt:");
    io::stdin().read_line(&mut buf)?;
    buf.remove(buf.len() - 1);
    let opt = buf.parse::<i32>().expect("Wrong input");
    if opt == 1 {
        Ok(true)
    } else if opt == 2 {
        Ok(false)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Wrong option"))
    }
}

fn get_simple_dividers(n: u32, f_n: u32) -> Vec<u32> {
    let mut sd = Vec::new();
    for i in 2..n {
        if num_integer::gcd(f_n, i) == 1 {
            sd.push(i);
        }
    }
    sd
}

fn get_random_sd(sd: Vec<u32>) -> u32 {
    let mut rng = rand::thread_rng();
    let id: usize = rng.gen::<usize>() % sd.len();
    sd[id]
}

fn pow_by_mod(c: u32, d: u32, n: u32) -> u8 {
    let mut res = 1;
    for _i in 1..=d {
        res = (res * c) % n;
    }
    res as u8
}

fn main() {
    if option().expect("Wrong input") {
        let pq = enter_p_q();
        let s = enter_str();
        // let vec = s.chars().map(|c| c as u8).collect::<Vec<u8>>();
        // for i in 0..vec.len(){
        //     print!("{} ", vec[i]);
        // }
        let n = pq.0 * pq.1;
        let f_n = (pq.0 - 1) * (pq.1 - 1);
        let e = get_random_sd(get_simple_dividers(n, f_n));
        println!("Public key: ({},{})", e, n);
        let mut k = 0;
        let d: u32;
        loop {
            k += 1;
            if (k * f_n + 1) % e == 0 {
                d = (k * f_n + 1) / e;
                break;
            }
        }
        println!("Private key: ({},{})", d, n);
        let mut enc_msg: Vec<u8> = Vec::new();
        for c in s.chars() {
            enc_msg.push(pow_by_mod(c as u32, e, n));
        }
        for num in enc_msg {
            print!("{} ", num);
        }
        println!()
    } else {
        let dn = enter_d_n();
        let encr_s = enter_encr_str();
        let d = dn.0;
        let n = dn.1;
        let s = encr_s
            .iter()
            .map(|c| (pow_by_mod(*c as u32, d, n) as char))
            .collect::<String>();
        println!("{}", s);
    }
}
