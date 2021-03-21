use crate::git_object::*;
use std::{fs, path::Path};

#[derive(Default)]
struct HashObjectOption<'a> {
    write_object: bool,
    filename: Option<&'a String>,
}

pub fn hash_object(commands: &[String]) {
    let option = parse_options(commands);
    if option.filename.is_none() {
        return;
    }

    let content = load_file(&option.filename.as_ref().unwrap());
    let blob = GitObject::Blob(content);
    let sha1hash = blob.to_hash_str();
    if option.write_object {
        blob.write().expect("failed to write");
    }
    println!("{}", sha1hash);
}

fn load_file(filename: &str) -> Vec<u8> {
    let object_path = Path::new(&filename);
    fs::read(object_path).unwrap()
}

fn parse_options(commands: &[String]) -> HashObjectOption {
    let mut option: HashObjectOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            "-w" => option.write_object = true,
            _ => option.filename = Some(&token),
        }
    }
    option
}
