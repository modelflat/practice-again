
pub fn parity_bit(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |a, &b| a ^ b)
}
