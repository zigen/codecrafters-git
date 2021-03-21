use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::io::Write;

pub fn hash_to_path_str(hash: &str) -> String {
    format!(".git/objects/{}/{}", &hash[0..2], &hash[2..])
}

pub fn hash_to_str(hash: &[u8]) -> String {
    hash.iter()
        .map(|n| format!("{:02x}", n))
        .collect::<String>()
}

pub fn hash(content: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(content);
    hasher.finalize().to_vec()
}

pub fn compress(content: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut e = ZlibEncoder::new(&mut buf, Compression::default());
    e.write_all(content).unwrap();
    let bytes = e.finish().unwrap();
    // unsafe {
    //     print!("{}", String::from_utf8_unchecked(bytes.to_vec()));
    // }
    bytes.to_vec()
}
