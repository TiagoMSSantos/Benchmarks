// use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Position {
    x: i8,
    y: i8,
}

impl Position {
    fn update_x(&mut self) {
        self.x += 1;
    }

    fn update_y(&mut self) {
        self.y += 1;
        self.update_x();
    }
}

fn hello_world() {
    let x = 10;
    let mut y :&i8 = &x;
    println!("Hello, world: {}", y);
    y = &2;
    println!("Hello, world: {}", y);

    let mut pos = Position {x:1, y:2};
    pos.x = 3;
    pos.update_x();
    Position::update_y(&mut pos);

    println!("pos: {{ {:#?} }}", pos);
    let pos2 = &mut pos;
    pos2.x = 12;
    let pos3 = &pos2;

    println!("pos: {{ {:#?} }}", pos2);
    println!("pos: {{ {:#?} }}", pos3);
}


fn read_file() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let file = File::open("../input_2.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn main() {
    hello_world();
    let file_text = read_file();
    println!("file: {:#?}", file_text);

    let text = file_text.unwrap();
    let mut lines = text.lines();

    let line1 = lines.next().unwrap();
    let mut chars_line1: Vec<char> = line1.chars().collect();
    chars_line1.sort();
    println!("line 1: {:#?}", line1);
    println!("line 1: {:#?}", chars_line1);

    let line2 = lines.next().unwrap();
    let mut chars_line2: Vec<char> = line2.chars().collect();
    chars_line2.sort();
    println!("line 2: {:#?}", line2);
    println!("line 2: {:#?}", chars_line2);

    let matching = chars_line1.iter().zip(&chars_line2).filter(|&(a, b)| a == b).count();
    println!("{}", matching == chars_line1.len() && matching == chars_line2.len());
}
