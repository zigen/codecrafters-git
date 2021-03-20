use crate::git_object::*;
use crate::utils::*;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::Path};

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
    let blob = GitObject::Blob(&content);
    let blob_content = blob.to_string();
    let sha1hash = hash(blob_content.as_bytes());
    if option.write_object {
        let compressed = compress(blob_content.as_bytes());
        let path_str = hash_to_path_str(&sha1hash);
        let path = Path::new(&path_str);
        if !path.exists() {
            let builder = fs::DirBuilder::new();
            builder.create(path.parent().unwrap()).unwrap();
            let mut f = fs::File::create(&path).unwrap();
            f.write_all(&compressed).unwrap();
        }
    }
    println!("{}", sha1hash);
}

fn hash(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();
    hash_to_str(&result)
}

fn load_file(filename: &str) -> Vec<u8> {
    let object_path = Path::new(&filename);
    fs::read(object_path).unwrap()
}

fn compress(content: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut e = ZlibEncoder::new(&mut buf, Compression::default());
    e.write_all(content).unwrap();
    let bytes = e.finish().unwrap();
    // unsafe {
    //     print!("{}", String::from_utf8_unchecked(bytes.to_vec()));
    // }
    bytes.to_vec()
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
