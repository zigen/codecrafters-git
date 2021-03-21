use crate::git_object::*;

#[derive(Default)]
struct LsTreeOption<'a> {
    name_only: bool,
    obj_hash: Option<&'a String>,
}

pub fn ls_tree(commands: &[String]) {
    let option = parse_options(commands);
    let hash = option.obj_hash.expect("no object hash given");

    let result = match load_object_by_hash(hash) {
        Ok(r) => r,
        Err(e) => panic!("{:?}", e),
    };

    match &result {
        GitObject::Blob(_) => {
            println!("not a tree object");
        }
        GitObject::Tree(lst) => {
            if option.name_only {
                for e in lst {
                    println!("{}", e.filename);
                }
            } else {
                result.pretty_print();
            }
        }
    }
}

fn parse_options(commands: &[String]) -> LsTreeOption {
    let mut option: LsTreeOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            "--name-only" => option.name_only = true,
            _ if token.len() == 40 => option.obj_hash = Some(&token),
            _ => println!("ignore option {}", token),
        }
    }
    option
}
