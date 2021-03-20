#[derive(Debug)]
pub enum GitObject<'a> {
    Blob(&'a [u8]),
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

impl<'a> GitObject<'a> {
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
