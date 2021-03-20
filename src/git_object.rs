#[derive(Debug)]
pub enum GitObject<'a> {
    Blob(&'a [u8]),
    Tree,
}

impl<'a> GitObject<'a> {
    pub fn pretty_print(&self) {
        match self {
            GitObject::Blob(s) => print!("{}", String::from_utf8_lossy(s)),
            GitObject::Tree => println!("tree"),
        }
    }
    pub fn get_content(&self) -> Vec<u8> {
        match self {
            GitObject::Blob(s) => s.to_vec(),
            GitObject::Tree => vec![],
        }
    }
    pub fn size(&self) -> usize {
        match self {
            GitObject::Blob(s) => s.len(),
            GitObject::Tree => 40,
        }
    }
    pub fn type_name(&self) -> String {
        match self {
            GitObject::Blob(_) => String::from("blob"),
            GitObject::Tree => String::from("tree"),
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
