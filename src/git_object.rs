use crate::utils::*;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum GitObject {
    Blob(Vec<u8>),
    Tree(Vec<GitTreeNode>),
}

#[derive(Debug)]
pub struct GitTreeNode {
    pub mode: u32,
    pub node_type: GitNodeType,
    pub hash: String,
    pub filename: String,
}

#[derive(Debug)]
pub enum GitNodeType {
    Blob,
    Tree,
}

impl GitObject {
    pub fn pretty_print(&self) {
        match self {
            GitObject::Blob(s) => print!("{}", String::from_utf8_lossy(s)),
            GitObject::Tree(lst) => {
                for e in lst {
                    println!("{} {} {}\t{}", e.mode, e.type_name(), e.hash, e.filename)
                }
            }
        }
    }

    pub fn get_content(&self) -> Vec<u8> {
        match self {
            GitObject::Blob(s) => s.to_vec(),
            GitObject::Tree(_) => vec![],
        }
    }
    pub fn size(&self) -> usize {
        match self {
            GitObject::Blob(s) => s.len(),
            GitObject::Tree(_) => 40,
        }
    }
    pub fn type_name(&self) -> String {
        match self {
            GitObject::Blob(_) => String::from("blob"),
            GitObject::Tree(_) => String::from("tree"),
        }
    }
    pub fn to_node_type(&self) -> GitNodeType {
        match self {
            GitObject::Blob(_) => GitNodeType::Blob,
            GitObject::Tree(_) => GitNodeType::Tree,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {}\0{}",
            self.type_name(),
            self.size(),
            self.get_content()
                .iter()
                .map(|r| *r as char)
                .collect::<String>()
        )
    }
}

impl GitTreeNode {
    pub fn new(filename: String, hash: String, mode: u32, node_type: GitNodeType) -> Self {
        GitTreeNode {
            filename,
            hash,
            mode,
            node_type,
        }
    }

    pub fn type_name(&self) -> String {
        match self.node_type {
            GitNodeType::Blob => String::from("blob"),
            GitNodeType::Tree => String::from("tree"),
        }
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
    GitObject::Blob(blob.to_vec())
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

pub fn load_object_by_hash(hash: &str) -> GitObject {
    let obj = load_obj_file(hash);
    parse(&obj)
}
