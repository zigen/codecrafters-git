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

pub fn str_to_hash(s: String) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|buf| String::from_utf8(buf.to_vec()).unwrap())
        .map(|buf| u8::from_str_radix(&buf, 16).unwrap())
        .collect::<Vec<u8>>()
}

pub fn compress(content: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut e = ZlibEncoder::new(&mut buf, Compression::default());
    e.write_all(content).unwrap();
    let bytes = e.finish().unwrap();
    bytes.to_vec()
}

#[cfg(test)]
mod test {
    use crate::utils::*;

    #[test]
    fn test_str_to_hash() {
        let s = String::from("341d422eca9785ce3f93590d66bda0a47facb5d9");
        let h = str_to_hash(s);
        assert_eq!(h[0], 52);
        assert_eq!(h[1], 29);
        assert_eq!(h[2], 66);
        assert_eq!(h[3], 46);
        assert_eq!(h.len(), 20);
    }
}
