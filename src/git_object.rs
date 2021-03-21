use crate::utils::*;
use flate2::read::ZlibDecoder;
use std::{fs, io, io::Read, io::Write, path::Path, result};

#[derive(Debug)]
pub enum GitObject {
    Blob(Vec<u8>),
    Tree(Vec<GitTreeNode>),
}

#[derive(Debug)]
pub struct GitTreeNode {
    pub mode: u32,
    pub node_type: GitNodeType,
    pub hash: Vec<u8>,
    pub filename: String,
}

#[derive(Debug)]
pub enum GitNodeType {
    Blob,
    Tree,
}

#[derive(Debug)]
pub enum GitObjectError {
    IOError(io::Error),
    CustomIOError(String),
    ParseError(String),
    WriteError(String),
    NotImplementError(String),
}

type Result<T> = result::Result<T, GitObjectError>;

impl GitObject {
    pub fn new_tree(mut nodes: Vec<GitTreeNode>) -> Self {
        nodes.sort_by(|a, b| a.filename.cmp(&b.filename));
        GitObject::Tree(nodes)
    }

    pub fn pretty_print(&self) {
        match self {
            GitObject::Blob(s) => print!("{}", String::from_utf8_lossy(s)),
            GitObject::Tree(lst) => {
                for e in lst {
                    println!(
                        "{} {} {}\t{}",
                        e.mode,
                        e.type_name(),
                        hash_to_str(&e.hash),
                        e.filename
                    )
                }
            }
        }
    }

    pub fn get_content(&self) -> Vec<u8> {
        match self {
            GitObject::Blob(s) => s.to_vec(),
            GitObject::Tree(t) => {
                t.iter()
                    .flat_map(|e| {
                        let mut v: Vec<u8> =
                            format!("{} {}\0", e.mode, e.filename).as_bytes().to_vec();
                        // println!("file: {} hash: {:02X?}", e.filename, e.hash.to_vec());
                        v.append(&mut e.hash.to_vec());
                        v
                    })
                    .collect::<Vec<u8>>()
            }
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
        let content = self.get_content();
        format!(
            "{} {}\0{}",
            self.type_name(),
            content.len(),
            content.iter().map(|r| *r as char).collect::<String>()
        )
    }

    pub fn to_node_content(&self) -> Vec<u8> {
        let mut content = self.get_content();
        let mut v: Vec<u8> = format!("{} {}\0", self.type_name(), content.len())
            .as_bytes()
            .to_vec();
        v.append(&mut content);
        v
    }

    pub fn to_hash(&self) -> Vec<u8> {
        hash(self.to_string().as_bytes())
    }

    pub fn to_hash_str(&self) -> String {
        hash_to_str(&hash(&self.to_node_content()))
    }
    pub fn write(&self) -> Result<()> {
        let hash = self.to_hash_str();
        // println!(
        //     "write****\nstr: {:02X?}\nhash: {:02X?}",
        //     self.to_node_content(),
        //     self.get_content()
        // );
        let compressed = compress(&self.to_node_content());
        let path_str = hash_to_path_str(&hash);
        let path = Path::new(&path_str);

        if !path.exists() {
            let parent_path = match path.parent() {
                Some(p) => p,
                None => {
                    return Err(GitObjectError::WriteError(format!(
                        "parent path does not exist. {}",
                        path.display()
                    )))
                }
            };
            if !parent_path.exists() {
                let builder = fs::DirBuilder::new();
                builder.create(path.parent().unwrap()).unwrap();
            }
            let mut f = fs::File::create(&path).unwrap();
            f.write_all(&compressed).expect("failed to write");
        }
        Ok(())
    }
}

impl GitTreeNode {
    pub fn new(filename: String, hash: &[u8], mode: u32, node_type: GitNodeType) -> Self {
        GitTreeNode {
            filename,
            hash: hash.to_vec(),
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

fn parse(content: &[u8]) -> Result<GitObject> {
    let content_str = String::from_utf8_lossy(content);
    if content_str.starts_with("blob") {
        return parse_blob(content);
    }
    if content_str.starts_with("tree") {
        return parse_tree(content);
    }

    panic!("unknown content: {:?}", content);
}

fn parse_blob(content: &[u8]) -> Result<GitObject> {
    if content.len() < 6 {
        return Err(GitObjectError::ParseError(format!(
            "blob too short. {:?}",
            content
        )));
    }
    let s = &content[5..];
    if let Some(i) = s.iter().position(|&e| e == 0) {
        let blob = &s[(i + 1)..];
        Ok(GitObject::Blob(blob.to_vec()))
    } else {
        Err(GitObjectError::ParseError(format!(
            "blob too short. {:?}",
            content
        )))
    }
}

fn parse_tree(content: &[u8]) -> Result<GitObject> {
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
        let hash = &blob[name_index + 1..hash_index];
        let hash_str = hash_to_str(hash);

        let obj = match load_obj_file(&hash_str) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        let content = match parse(&obj) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        v.push(GitTreeNode::new(
            filename.clone(),
            hash,
            mode,
            content.to_node_type(),
        ));
        if hash_index >= blob.len() {
            break;
        }
        blob = &blob[hash_index..];
    }
    Ok(GitObject::Tree(v))
}

fn load_obj_file(hash: &str) -> Result<Vec<u8>> {
    let path_str = hash_to_path_str(&hash);
    let object_path = Path::new(&path_str);
    if !object_path.exists() {
        return Err(GitObjectError::CustomIOError(format!(
            "the file {} does not exists.",
            object_path.display()
        )));
    }
    let file_content = fs::read(object_path)
        .unwrap_or_else(|e| panic!("failed to read: {}. path: {}", e, object_path.display()));
    let mut d = ZlibDecoder::new(&*file_content);
    let mut buf = vec![];
    match d.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(GitObjectError::IOError(e)),
    }
}

pub fn load_object_by_hash(hash: &str) -> Result<GitObject> {
    match load_obj_file(hash) {
        Ok(obj) => parse(&obj),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use crate::git_object::*;

    #[test]
    fn test_blob_hash() {
        let o = GitObject::Blob(String::from("hogehoge\n").as_bytes().to_vec());
        println!("{}", o.to_string());
        assert_eq!(o.to_string(), "blob 9\0hogehoge\n");
        assert_eq!(o.to_hash_str(), "e9bc11025c28829eedf6d30cd3b65628648cad5f");
    }

    #[test]
    fn test_tree_hash() {
        let o = GitObject::Blob(String::from("hogehoge\n").as_bytes().to_vec());
        let t = GitObject::Tree(vec![GitTreeNode::new(
            "hogehoge".to_string(),
            &o.to_hash(),
            100644,
            GitNodeType::Tree,
        )]);

        let hash = o.to_hash();
        t.pretty_print();

        let content = t.to_node_content();
        // println!("{}", String::from_utf8_lossy(&content.to_vec()));
        assert_eq!(content.to_vec()[..5], b"tree "[..]);
        assert_eq!(content.to_vec()[8..14], b"100644"[..]);
        assert_eq!(content.to_vec()[8..15], b"100644 "[..]);
        assert_eq!(content.to_vec()[15..23], b"hogehoge"[..]);
        assert_eq!(content.to_vec()[23], b'\0');
        assert_eq!(content.to_vec()[24..], hash[..]);
        assert_eq!(content.len(), 40);
        // assert_eq!(o.to_hash_str(), "e9bc11025c28829eedf6d30cd3b65628648cad5f");
    }
}
