/// FNV-1a 64-bit hashing algorithm.
///
/// Ref: http://www.isthe.com/chongo/tech/comp/fnv/
pub fn fnv1a_64(text: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
    let prime: u64 = 0x100000001b3; // FNV prime

    for byte in text {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(prime);
    }

    hash
}
