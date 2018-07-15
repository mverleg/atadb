//use sha2::{Sha256, Digest};
use rand;

/// Hashing should work on utf8 and binary data.
/// It should not be possibly to submit many different colliding values easily, or performance may suffer.
/// This is achieved by using a secret salt and relying on a fast non-cryptographic hashing function.
/// If the hashing fucntion is changed, all databases need to re-intern all their strings, so don't change it.
pub fn hash_utf8(txt: &str) -> u64 {
    //    let mut hasher = Sha256::default();
    //    hasher.input(txt.bytes());
    //    bytes_to_u64(hasher.result())
    unimplemented!()
}

pub fn hash_bin(bin: &Vec<u8>) -> u64 {
    //    let mut hasher = Sha256::default();
    //    hasher.input(txt);
    //    let bytes = hasher.result();
    //    bytes_to_u64(bytes);
    unimplemented!()
}

#[inline]
fn bytes_to_u64(bytes: &[u8; 8]) -> u64 {
    // https://stackoverflow.com/a/36676814/723090
    unimplemented!()
}

// TODO @mverleg: Benchmark this! Not sure there's a choice but this shouldn't be the bottleneck.

pub fn make_salt() -> Vec<u8> {
    let mut salt = Vec::with_capacity(8);
    for _ in 0..8 {
        salt.push(rand::random());
    }
    salt
}
