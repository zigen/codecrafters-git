use crate::git_object::*;
use crate::utils::*;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Default)]
struct CatFileOption<'a> {
    pretty_print: bool,
    show_size: bool,
    show_type: bool,
    obj_hash: Option<&'a String>,
}

pub fn cat_file(commands: &[String]) {
    let option = parse_options(commands);
    if option.obj_hash.is_none() {
        return;
    }

    let content = load_obj_file(&option.obj_hash.as_ref().unwrap());
    // println!("content:\n{}\n{:?}", content, content.as_bytes().to_vec());
    let result = parse(&content);
    // println!("result: {:?}", result);
    if option.pretty_print {
        result.pretty_print();
        return;
    }
    if option.show_size {
        println!("{:?}", result.size());
        return;
    }
    if option.show_type {
        println!("{}", result.type_name());
        return;
    }
}

fn parse(content: &[u8]) -> GitObject {
    let content_str = String::from_utf8_lossy(content);
    if content_str.starts_with("blob") {
        return parse_blob(content);
    }
    if content_str.starts_with("tree") {
        return parse_tree(content);
    }

    panic!("unknown content: {:?}", content);
}

fn parse_blob(content: &[u8]) -> GitObject {
    let s = &content[5..];
    let i = s.iter().position(|&e| e == 0).unwrap();
    let blob = &s[(i + 1)..];
    GitObject::Blob(blob)
}

fn parse_tree(content: &[u8]) -> GitObject {
    let s = &content[5..];
    let si = s.iter().position(|&e| e == 0).unwrap();
    let mut blob = &s[(si + 1)..];

    let mut v = vec![];
    loop {
        // println!("{} : {}", blob[i] as char, blob[i]);
        let mode_index = blob.iter().position(|&c| c == 32).unwrap();
        let name_index = blob.iter().position(|&c| c == 0).unwrap();
        let hash_index = name_index + 21;
        let mode = String::from_utf8(blob[..mode_index].to_vec())
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let filename = String::from_utf8(blob[mode_index + 1..name_index].to_vec()).unwrap();
        let hash_str = hash_to_str(&blob[name_index + 1..hash_index]);

        let obj = load_obj_file(&hash_str);
        let content = parse(&obj);

        v.push(GitTreeNode::new(
            filename.clone(),
            hash_str.clone(),
            mode,
            content.to_node_type(),
        ));
        if hash_index >= blob.len() {
            break;
        }
        blob = &blob[hash_index..];
    }
    GitObject::Tree(v)
}

fn load_obj_file(hash: &str) -> Vec<u8> {
    let path_str = hash_to_path_str(&hash);
    let object_path = Path::new(&path_str);
    // println!("{}", object_path.to_str().unwrap());
    let file_content = fs::read(object_path).unwrap();
    let mut d = ZlibDecoder::new(&*file_content);
    let mut result = vec![];
    let _size = d.read_to_end(&mut result).unwrap();
    result
}

fn parse_options(commands: &[String]) -> CatFileOption {
    let mut option: CatFileOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            "-p" => option.pretty_print = true,
            "-s" => option.show_size = true,
            "-t" => option.show_type = true,
            _ if token.len() == 40 => option.obj_hash = Some(&token),
            _ => println!("ignore option {}", token),
        }
    }
    option
}
