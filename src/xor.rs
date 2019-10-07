use std::iter::Iterator;

/// Performs XOR encoding on an array of bytes, using a given key.
pub fn xor_inplace(bytes: &mut [u8], key: &[u8]) {
    for (i, b) in bytes.iter_mut().enumerate() {
        *b ^= key[i % key.len()];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_xor_applied_twice() {
        let mut text = "some text".to_string().into_bytes();

        let key = "abc";

        xor_inplace(&mut text, key.as_bytes());
        xor_inplace(&mut text, key.as_bytes());

        assert_eq!(String::from_utf8_lossy(&text), "some text".to_string());
    }
}
