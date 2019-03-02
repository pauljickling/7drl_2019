use std::io;
use std::process::Command;

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

    // clears screen
    print!("{}[2J", 27 as char);

    println!("What is your name?");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => println!("Name: {}", input),
        Err(error) => println!("error: {}", error),
    }
    // print!("{}", _files);

    print!("{}[2J", 27 as char);

    let mut two_d_dungeon = String::from("###@###");
    println!("\t{}", two_d_dungeon);
    println!("\n\n");
    println!("Name: {}Class: Hunter  Lvl: 1  HP: 20", input);

    // this is how you handle game input
    // let mut byteInput = stdin.bytes();
}
