use crate::git_object::*;
use std::{fs, path::Path};

#[derive(Default)]
struct WriteTreeOption {}

pub fn write_tree(commands: &[String]) {
    let _option = parse_options(commands);
    let path = Path::new(".");
    let nodes = write_tree_rec(&path);
    // println!("nodes: {:?}", nodes);
    let tree = GitObject::new_tree(nodes);
    tree.write().unwrap();
    println!("{}", tree.to_hash_str());
}

fn write_tree_rec(path: &Path) -> Vec<GitTreeNode> {
    path.read_dir()
        .expect("read_dir call failed")
        .filter_map(|entry| match entry {
            Ok(entry) if !entry.path().starts_with("./.git") => {
                // println!("{:?}", entry.path());
                if entry.file_type().unwrap().is_dir() {
                    let tree = GitObject::Tree(write_tree_rec(&entry.path()));
                    Some(GitTreeNode::new(
                        entry.file_name().into_string().unwrap(),
                        &tree.to_hash(),
                        100644,
                        GitNodeType::Tree,
                    ))
                } else {
                    // println!("read blob: {}", entry.path().display());
                    let content = fs::read(entry.path()).unwrap();
                    let blob = GitObject::Blob(content);
                    blob.write().unwrap();
                    Some(GitTreeNode::new(
                        entry.file_name().into_string().unwrap(),
                        &blob.to_hash(),
                        100644,
                        GitNodeType::Blob,
                    ))
                }
            }
            _ => None,
        })
        .collect()
}

fn parse_options(commands: &[String]) -> WriteTreeOption {
    let option: WriteTreeOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            _ => println!("ignore option {}", token),
        }
    }
    option
}
