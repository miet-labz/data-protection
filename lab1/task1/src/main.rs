use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let mut path = String::new();
    let mut buffer = Vec::new();
    let mut map = HashMap::new();
    println!("Enter filename: ");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
    let mut file = File::open(&path[0..path.len() - 1])?;

    //заполняем map элементами {i: 0}
    for i in 0u8..=255 {
        map.insert(i, 0u32);
    }

    //считываем файл побитно
    file.read_to_end(&mut buffer)?;

    println!("File Length: {}", &buffer.len());

    //проходимся по строке битов и записываем количество каждого бита в map
    for byte in &buffer {
        *map.get_mut(&byte).unwrap() += 1;
    }

    // вывод map
    // 1-ый вариант
    for i in 0u8..=255 {
        println!("{}: {}", i, map[&i]);
    }

    // 2-ой вариант
    for i in 0u8..=255 {
        println!(
            "{}: {:.2}%",
            i,
            map[&i] as f32 / buffer.len() as f32 * 100f32
        );
    }

    //очистка
    buffer.clear();
    map.clear();
    Ok(())
}
