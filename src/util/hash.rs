
use sha2::{Sha256, Digest};

/// Hashing should work on utf8 and binary data.
/// It should be cryptographically secure (otherwise an adversary
/// could seriously degrade performance with collisions).
/// The hashes should also be stable, as they are stored to files.
/// Note that using a non-crypto hash and crypto-hashing the result
/// will be faster, but won't prevent against collisions at all
/// (only against reversing, which isn't the concern in this case).
pub fn hash_utf8(txt: &str) -> u64 {
    let mut hasher = Sha256::default();
    hasher.input(&txt.bytes());
    bytes_to_u64(hasher.result())
}

pub fn hash_bin(bin: &Vec<u8>) -> u64 {
//    let mut hasher = Sha256::default();
//    hasher.input(txt);
//    let bytes = hasher.result();
//    bytes_to_u64(bytes);
    0
}

#[inline]
fn bytes_to_u64(bytes: &[u8; 8]) -> u64 {
    // https://stackoverflow.com/a/36676814/723090
}

// TODO @mverleg: Benchmark this! Not sure there's a choice but this shouldn't be the bottleneck.
