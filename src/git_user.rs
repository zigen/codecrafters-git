#[derive(Debug)]
pub struct User {
    pub name: &'static str,
    pub email: &'static str,
}

pub static USER: User = User {
    name: "zigen",
    email: "zigen@horol.org",
};
