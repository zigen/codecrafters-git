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

#[derive(Debug)]
enum GitObject {
    Blob(String),
    Tree,
}

impl GitObject {
    pub fn pretty_print(&self) {
        match self {
            GitObject::Blob(s) => println!("{}", s),
            GitObject::Tree => println!("tree"),
        }
    }
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
    }
}

fn parse(content: &str) -> GitObject {
    if content.starts_with("blob") {
        return parse_blob(content);
    }
    if content.starts_with("tree") {
        return GitObject::Tree;
    }

    panic!("unknown content: {}", content);
}

fn parse_blob(content: &str) -> GitObject {
    let s = &content[5..];
    let blob = match s.find('\0') {
        Some(i) => {
            let size = s[0..i].parse::<usize>().unwrap();
            &s[(i + 1)..(i + size)]
        }
        None => "",
    };
    GitObject::Blob(blob.to_string())
}

fn load_obj_file(hash: &str) -> String {
    let path_str = format!(".git/objects/{}/{}", &hash[0..2], &hash[2..]);
    let object_path = Path::new(&path_str);
    // println!("{}", object_path.to_str().unwrap());
    let file_content = fs::read(object_path).unwrap();
    let mut d = ZlibDecoder::new(&*file_content);
    let mut result = String::new();
    let _size = d.read_to_string(&mut result).unwrap();
    result
}

fn parse_options(commands: &[String]) -> CatFileOption {
    let mut option: CatFileOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            "-p" => option.pretty_print = true,
            _ if token.len() == 40 => option.obj_hash = Some(&token),
            _ => println!("ignore option {}", token),
        }
    }
    option
}
