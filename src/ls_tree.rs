use crate::git_object::*;
use crate::utils::*;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Default)]
struct LsTreeOption<'a> {
    name_only: bool,
    obj_hash: Option<&'a String>,
}

pub fn ls_tree(commands: &[String]) {
    let option = parse_options(commands);
    if option.obj_hash.is_none() {
        return;
    }

    let result = load_object_by_hash(&option.obj_hash.as_ref().unwrap());

    match &result {
        GitObject::Blob(_) => {
            println!("not a tree object");
            return;
        }
        GitObject::Tree(lst) => {
            if option.name_only {
                for e in lst {
                    println!("{}", e.filename);
                }
                return;
            } else {
                result.pretty_print();
            }
        }
    }
}

fn parse_options(commands: &[String]) -> LsTreeOption {
    let mut option: LsTreeOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            "--name-only" => option.name_only = true,
            _ if token.len() == 40 => option.obj_hash = Some(&token),
            _ => println!("ignore option {}", token),
        }
    }
    option
}
