use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    let b = c as u8;
                    if c.is_control() {
                        println!("{:03?} ({:#010b})\r", b, b);
                    } else {
                        println!("{:03?} ({:#010b}) ({})\r", b, b, c);
                    }
                },
                Key::Ctrl('q') => break,
                _ => println!("{:03?}\r", key),
            }
            Err(err) => die(err),
        }
    }
}
