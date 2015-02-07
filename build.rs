#![feature(io)]

use std::old_io::Command;
use std::old_io::process::InheritFd;

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let status = match cmd.stdout(InheritFd(1)).stderr(InheritFd(2)).status() {
        Ok(status) => status,
        Err(e) => panic!("failed to spawn process: {}", e),
    };
    if !status.success() {
        panic!("nonzero exit status: {}", status);
    }
}

fn main() {
    let mut fetch_src = Command::new("./build.sh");

    run(&mut fetch_src);
}
