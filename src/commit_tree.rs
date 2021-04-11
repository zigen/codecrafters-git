use crate::git_object::*;
use crate::git_user::USER;
use std::{fs, path::Path};

#[derive(Default, Debug)]
struct CommitTreeOption<'a> {
    tree_sha: Option<&'a String>,
    parent_sha: Option<&'a String>,
    message: Option<&'a String>,
}

pub fn commit_tree(commands: &[String]) {
    let option = parse_options(commands);
    println!("option: {:?}", option);
    let commit = GitObject::new_commit(
        option.tree_sha.unwrap().to_string(),
        option.parent_sha,
        &USER,
        &USER,
        option.message.unwrap().to_string(),
    );
    println!("{}", commit.to_string());
    // let nodes = write_tree_rec(&path);
    // println!("nodes: {:?}", nodes);
    // let tree = GitObject::new_tree(nodes);
    // tree.write().unwrap();
    // println!("{}", tree.to_hash_str());
}

// fn write_tree_rec(path: &Path) -> Vec<GitTreeNode> {
//     path.read_dir()
//         .expect("read_dir call failed")
//         .filter_map(|entry| match entry {
//             Ok(entry) if !entry.path().starts_with("./.git") => {
//                 // println!("{:?}", entry.path());
//                 if entry.file_type().unwrap().is_dir() {
//                     let tree = GitObject::Tree(write_tree_rec(&entry.path()));
//                     Some(GitTreeNode::new(
//                         entry.file_name().into_string().unwrap(),
//                         &tree.to_hash(),
//                         100644,
//                         GitNodeType::Tree,
//                     ))
//                 } else {
//                     // println!("read blob: {}", entry.path().display());
//                     let content = fs::read(entry.path()).unwrap();
//                     let blob = GitObject::Blob(content);
//                     blob.write().unwrap();
//                     Some(GitTreeNode::new(
//                         entry.file_name().into_string().unwrap(),
//                         &blob.to_hash(),
//                         100644,
//                         GitNodeType::Blob,
//                     ))
//                 }
//             }
//             _ => None,
//         })
//         .collect()
// }

fn parse_options(commands: &[String]) -> CommitTreeOption {
    let mut option: CommitTreeOption = Default::default();
    let mut message_flag = false;
    let mut parent_sha_flag = false;
    for token in &commands[2..] {
        match &token[..] {
            "-m" => message_flag = true,
            "-p" => parent_sha_flag = true,
            _ if token.len() == 40 => {
                if parent_sha_flag {
                    option.parent_sha = Some(&token);
                    parent_sha_flag = false;
                } else {
                    option.tree_sha = Some(&token);
                }
            }
            _ => {
                if message_flag {
                    option.message = Some(&token);
                    message_flag = false;
                } else {
                    println!("ignore option {}", token);
                }
            }
        }
    }
    option
}
