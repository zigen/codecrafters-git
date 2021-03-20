pub fn hash_to_path_str(hash: &str) -> String {
    format!(".git/objects/{}/{}", &hash[0..2], &hash[2..])
}

pub fn hash_to_str(hash: &[u8]) -> String {
    hash.iter()
        .map(|n| format!("{:02x}", n))
        .collect::<String>()
}
