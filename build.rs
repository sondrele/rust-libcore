#![feature(io)]

use std::old_io::Command;

fn main() {
    match Command::new("./build.sh").status() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    }
}
