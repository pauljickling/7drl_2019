extern crate termion;

use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};
use std::process::Command;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

fn main() {
    let generate = Command::new("src/generate/proc.o")
        .output()
        .expect("failed to execute process");

    let mut dungeon = String::new();
    for c in generate.stdout.as_slice() {
        if c.is_ascii() == true {
            dungeon.push(*c as char);
        }
    }

    println!("{}", clear::All);
    println!("{}What is your name?", cursor::Goto(1, 1));

    let mut name = String::new();
    match stdin().read_line(&mut name) {
        Ok(_n) => println!("Name: {}", name),
        Err(error) => println!("error: {}", error),
    }

    println!("{}", clear::All);

    // Enter Raw Mode
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Roguelike Prototype. Press q to quit.",
        cursor::Goto(1, 1),
        color::Fg(color::LightGreen)
    )
    .unwrap();
    // Cursor Definitions
    let mut height: u16 = 3;
    let mut width: u16 = 1;
    let mut player_position: (u16, u16) = (0, 0);

    // Dungeon Definitions
    let mut dungeon_map = HashMap::new();
    let mut dungeon_line = String::new();
    let mut old_square = '.';
    write!(stdout, "{}", cursor::Goto(1, 4)).unwrap();
    for c in dungeon.chars() {
        if c == '\n' {
            height += 1;
            width = 1;
            writeln!(stdout, "{}", dungeon_line).unwrap();
            dungeon_line.clear();
            writeln!(stdout, "{}", cursor::Goto(1, height)).unwrap();
        } else {
            if c == '@' {
                let player_height = height + 1;
                player_position = (width, player_height);
            }
            dungeon_line.push(c);
            dungeon_map.insert((width, height), c);
            width += 1;
        }
    }
    // controls
    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char('q') => break,
            Key::Left => {
                let new_pos = move_player(player_position, "left");
                let cur_height: i16 = new_pos.1 - 1;
                let new_square = dungeon_map.get(&(new_pos.0 as u16, cur_height as u16));
                if is_valid_move(*new_square.unwrap()) == true {
                    writeln!(
                        stdout,
                        "{}@",
                        cursor::Goto(new_pos.0 as u16, new_pos.1 as u16)
                    )
                    .unwrap();
                    writeln!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(player_position.0, player_position.1),
                        old_square,
                        cursor::Goto(new_pos.0 as u16, cur_height as u16)
                    )
                    .unwrap();
                    player_position = (new_pos.0 as u16, new_pos.1 as u16);
                    old_square = *new_square.unwrap();
                }
            }
            Key::Right => {
                let new_pos = move_player(player_position, "right");
                let cur_height = new_pos.1 - 1;
                let new_square = dungeon_map.get(&(new_pos.0 as u16, cur_height as u16));
                if is_valid_move(*new_square.unwrap()) == true {
                    writeln!(
                        stdout,
                        "{}@",
                        cursor::Goto(new_pos.0 as u16, new_pos.1 as u16)
                    )
                    .unwrap();
                    writeln!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(player_position.0, player_position.1),
                        old_square,
                        cursor::Goto(new_pos.0 as u16, cur_height as u16)
                    )
                    .unwrap();
                    player_position = (new_pos.0 as u16, new_pos.1 as u16);
                    old_square = *new_square.unwrap();
                }
            }
            Key::Up => {
                let new_pos = move_player(player_position, "up");
                let cur_height = new_pos.1 - 1;
                let new_square = dungeon_map.get(&(new_pos.0 as u16, cur_height as u16));
                if is_valid_move(*new_square.unwrap()) == true {
                    writeln!(
                        stdout,
                        "{}@",
                        cursor::Goto(new_pos.0 as u16, new_pos.1 as u16)
                    )
                    .unwrap();
                    writeln!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(player_position.0, player_position.1),
                        old_square,
                        cursor::Goto(new_pos.0 as u16, cur_height as u16)
                    )
                    .unwrap();
                    player_position = (new_pos.0 as u16, new_pos.1 as u16);
                    old_square = *new_square.unwrap();
                }
            }
            Key::Down => {
                let new_pos = move_player(player_position, "down");
                let cur_height = new_pos.1 - 1;
                let new_square = dungeon_map.get(&(new_pos.0 as u16, cur_height as u16));
                if is_valid_move(*new_square.unwrap()) == true {
                    writeln!(
                        stdout,
                        "{}@",
                        cursor::Goto(new_pos.0 as u16, new_pos.1 as u16)
                    )
                    .unwrap();
                    writeln!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(player_position.0, player_position.1),
                        old_square,
                        cursor::Goto(new_pos.0 as u16, cur_height as u16)
                    )
                    .unwrap();
                    player_position = (new_pos.0 as u16, new_pos.1 as u16);
                    old_square = *new_square.unwrap();
                }
            }
            _ => write!(stdout, "{}Oops!{}", cursor::Goto(1, 80), cursor::Goto(1, 1)).unwrap(),
        }
    }
    println!("Name: {}Class: Hunter  Lvl: 1  HP: 4", name);
    // exit process
    writeln!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}

fn move_player(old: (u16, u16), direction: &str) -> (i16, i16) {
    let mut direction_map = HashMap::new();
    direction_map.insert("left", (-1, 0));
    direction_map.insert("right", (1, 0));
    direction_map.insert("up", (0, -1));
    direction_map.insert("down", (0, 1));
    let adjust = match direction_map.get(direction) {
        Some(x) => x,
        None => panic!("Invalid direction provided for move function"),
    };
    let new_pos = (old.0 as i16 + adjust.0, old.1 as i16 + adjust.1);
    new_pos
}

fn is_valid_move(pos: char) -> bool {
    let mut check = true;
    let mut barriers = HashSet::new();
    barriers.insert('#');
    barriers.insert('+');
    if barriers.contains(&pos) {
        check = false;
    }
    check
}
