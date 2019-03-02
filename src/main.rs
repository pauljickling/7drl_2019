use std::process::Command;

fn main() {
    let generate = Command::new("src/generate/proc.o")
        .output()
        .expect("failed to execute process");

    let mut files = String::new();
    for c in generate.stdout.as_slice() {
        if c.is_ascii() == true {
            files.push(*c as char);
        }
    }
    println!("{}", files);
}
