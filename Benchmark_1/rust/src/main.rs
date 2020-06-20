use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rayon::prelude::*;

// #[derive(Debug)]
// struct Position {
//     x: i8,
//     y: i8,
// }

// #[allow(dead_code)]
// impl Position {
//     fn update_x(&mut self) {
//         self.x += 1;
//     }

//     fn update_y(&mut self) {
//         self.y += 1;
//         self.update_x();
//     }
// }

// #[allow(dead_code)]
// fn hello_world() {
//     let x = 10;
//     let mut y :&i8 = &x;
//     println!("Hello, world: {}", y);
//     y = &2;
//     println!("Hello, world: {}", y);

//     let mut pos = Position {x:1, y:2};
//     pos.x = 3;
//     pos.update_x();
//     Position::update_y(&mut pos);

//     println!("pos: {{ {:#?} }}", pos);
//     let pos2 = &mut pos;
//     pos2.x = 12;
//     let pos3 = &pos2;

//     println!("pos: {{ {:#?} }}", pos2);
//     println!("pos: {{ {:#?} }}", pos3);
// }


fn read_file() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let file : std::fs::File = File::open("../input_2.txt")?;
    let mut buf_reader : std::io::BufReader<File> = BufReader::new(file);
    let mut contents : String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    return Ok(contents);
}


fn main() {
    let file_text : Result<String, Box<dyn std::error::Error + 'static>> = read_file();

    let text : String = file_text.unwrap();
    let mut lines : std::str::Lines = text.lines();

    let line1 : &str = lines.next().unwrap();
    let mut chars_line1 : Vec<char> = line1.chars().collect::<Vec<char>>();
    chars_line1.par_sort_unstable();

    let line2 : &str = lines.next().unwrap();
    let mut chars_line2 : Vec<char> = line2.chars().collect();
    chars_line2.par_sort_unstable();

    let matching : usize = chars_line1.iter().zip(&chars_line2).filter(|&(a, b)| a == b).count();

    println!("RUST");
    println!("result1 = {}", matching == chars_line1.len() && matching == chars_line2.len());
}
