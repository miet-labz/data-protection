use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;

fn main() -> io::Result<()>{
    let mut path = String::new();
    let mut buffer = Vec::new();
    let mut map = HashMap::new();
    println!("Enter filename: ");
    io::stdin().read_line(&mut path).expect("Failed to read line");
    let mut file = File::open(&path[0..path.len()-1])?;

    //заполняем map элементами {i: 0}
    for i in 0u8..=255 {
        map.insert(i, 0u32);
    };

    //считываем файл побитно
    file.read_to_end(&mut buffer)?;

    println!("File Length: {}", &buffer.len());

    //проходимся по строке битов и записываем количество каждого бита в map
    for byte in &buffer{
        *map.get_mut(&byte).unwrap() += 1; 
    };

    //вывод map
    for i in 0u8..=255{
        println!("{}: {}", i, map[&i]);
    };

    //очистка
    buffer.clear();
    map.clear();
    Ok(())
}