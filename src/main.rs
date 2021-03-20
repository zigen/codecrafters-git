use git_starter_rust::cat_file::cat_file;
use git_starter_rust::hash_object::hash_object;
use git_starter_rust::ls_tree::ls_tree;
#[allow(unused_imports)]
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    execute_command(args)
}

fn execute_command(commands: Vec<String>) {
    match &*commands[1] {
        "init" => init(),
        "cat-file" => cat_file(&commands),
        "hash-object" => hash_object(&commands),
        "ls-tree" => ls_tree(&commands),
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
