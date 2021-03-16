#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    println!("Your code goes here!");

    let args: Vec<String> = env::args().collect();
    execute_command(args)
}

fn execute_command(commands: Vec<String>) {
    match &*commands[1] {
        "init" => init(),
        _ => println!("Initialized git directory"),
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}
