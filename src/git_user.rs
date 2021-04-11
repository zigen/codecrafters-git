#[derive(Debug)]
pub struct User {
    name: &'static str,
    email: &'static str,
}

pub static USER: User = User {
    name: "zigen",
    email: "zigen@horol.org",
};
