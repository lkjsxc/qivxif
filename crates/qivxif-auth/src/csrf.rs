use rand::{RngCore, rngs::OsRng};

pub fn generate_csrf_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

pub fn hash_csrf_token(token: &str) -> String {
    blake3::hash(token.as_bytes()).to_hex().to_string()
}

pub fn verify_csrf_token(token: &str, hash: &str) -> bool {
    hash_csrf_token(token) == hash
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(hex_char(byte >> 4));
        out.push(hex_char(byte & 0x0f));
    }
    out
}

fn hex_char(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + value - 10) as char,
        _ => unreachable!("nibble is bounded"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_tokens_are_distinct() {
        assert_ne!(generate_csrf_token(), generate_csrf_token());
    }

    #[test]
    fn verifies_matching_hash() {
        let token = generate_csrf_token();
        assert!(verify_csrf_token(&token, &hash_csrf_token(&token)));
    }

    #[test]
    fn rejects_different_token() {
        let token = generate_csrf_token();
        assert!(!verify_csrf_token("different", &hash_csrf_token(&token)));
    }
}
