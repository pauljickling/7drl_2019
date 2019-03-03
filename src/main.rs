extern crate termion;

use std::io;
use std::process::Command;
use termion::clear;

fn main() {
    let generate = Command::new("src/generate/proc.o")
        .output()
        .expect("failed to execute process");

    let mut _files = String::new();
    for c in generate.stdout.as_slice() {
        if c.is_ascii() == true {
            _files.push(*c as char);
        }
    }
    // println!("{}", _files);

    println!("{}", clear::All);

    println!("What is your name?");

    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_n) => println!("Name: {}", name),
        Err(error) => println!("error: {}", error),
    }
    // print!("{}", _files);

    println!("{}", clear::All);

    let mut _two_d_dungeon = String::from("###@###");
    println!("{}", _files);
    println!("\n\n");
    println!("Name: {}Class: Hunter  Lvl: 1  HP: 20", name);
}
