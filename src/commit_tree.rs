use crate::git_object::*;
use crate::git_user::USER;

#[derive(Default, Debug)]
struct CommitTreeOption<'a> {
    tree_sha: Option<&'a String>,
    parent_sha: Option<&'a String>,
    message: Option<&'a String>,
}

pub fn commit_tree(commands: &[String]) {
    let option = parse_options(commands);
    // println!("option: {:?}", option);
    let commit = GitObject::new_commit(
        option.tree_sha.unwrap().to_string(),
        option.parent_sha,
        &USER,
        &USER,
        option.message.unwrap().to_string(),
    );
    // println!("{}", commit.to_string());
    commit.write().unwrap();
    println!("{}", commit.to_hash_str());
}

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
