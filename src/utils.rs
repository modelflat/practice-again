use std::str::FromStr;

/// Most executables in this course should distinguish between Encoding and Decoding modes
pub enum OperationMode {
    Encrypt, Decrypt
}

impl FromStr for OperationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "encrypt" | "enc" | "e" => Ok(OperationMode::Encrypt),
            "decrypt" | "dec" | "d" => Ok(OperationMode::Decrypt),
            _ => Err(lower)
        }
    }
}

/// Detects whether an UTF-8 character belongs to a modern Russian character range.
#[inline]
pub fn is_russian_char(c: &char) -> bool {
    let c: u32 = (*c).into();
    (0x0410 <= c && c < 0x0450) || c == 0x0401 || c == 0x0451
}

/// Convenience wrapper.
pub fn is_russian_char_or_punct_or_num_or_ws(c: &char) -> bool {
    is_russian_char(c) || c.is_ascii_punctuation() || c.is_ascii_digit() || c.is_whitespace()
}
