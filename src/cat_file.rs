use crate::git_object::*;

#[derive(Default)]
struct CatFileOption<'a> {
    pretty_print: bool,
    show_size: bool,
    show_type: bool,
    obj_hash: Option<&'a String>,
}

pub fn cat_file(commands: &[String]) {
    let option = parse_options(commands);
    if option.obj_hash.is_none() {
        return;
    }

    let result = match load_object_by_hash(&option.obj_hash.as_ref().unwrap()) {
        Ok(r) => r,
        Err(e) => panic!("{:?}", e),
    };
    if option.pretty_print {
        result.pretty_print();
        return;
    }
    if option.show_size {
        println!("{:?}", result.size());
        return;
    }
    if option.show_type {
        println!("{}", result.type_name());
        return;
    }
}

fn parse_options(commands: &[String]) -> CatFileOption {
    let mut option: CatFileOption = Default::default();
    for token in &commands[2..] {
        match &token[..] {
            "-p" => option.pretty_print = true,
            "-s" => option.show_size = true,
            "-t" => option.show_type = true,
            _ if token.len() == 40 => option.obj_hash = Some(&token),
            _ => println!("ignore option {}", token),
        }
    }
    option
}
