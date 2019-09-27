/// simplest hash sum possible
pub fn parity_bit(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |a, &b| a ^ b)
}


pub mod sha {
    use std::convert::TryInto;

    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    const INIT: (
        u32, u32, u32, u32, u32, u32, u32, u32,
    ) = (
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    );

    /// Direct implementation of the pseudocode from Wikipedia :/
    /// Also check this for explanation, step-by-step computation and stuff:
    /// https://csrc.nist.gov/csrc/media/publications/fips/180/2/archive/2002-08-01/documents/fips180-2.pdf
    pub fn sha256(data: Vec<u8>) -> String {
        let (mut h0, mut h1, mut h2, mut h3, mut h4, mut h5, mut h6, mut h7) = INIT;
        let mut data = data;
        let l: u64 = (data.len() * 8) as u64;

        data.push(0x80u8);

        let n_bits = (data.len() + 8) * 8;

        let need_more_bits = if (n_bits) % 512 == 0 { 0 } else { 512 - n_bits % 512 };
        if need_more_bits != 0 {
            // need_more_bits guaranteed to be divisible by 8 here because n % 512 is div. by
            // 8 if n is div by 8, and n is data.len() * 8; 512 - (something div. by 8) is
            // also divisible by 8
            for _ in 0..(need_more_bits / 8) {
                data.push(0x00u8)
            }
        }

        data.extend(&l.to_be_bytes());

        let mut w: [u32; 64] = [0; 64];

        for chunk in data.chunks(64) {
            for (i, word) in chunk.chunks(4).enumerate() {
                w[i] = u32::from_be_bytes(word.try_into().unwrap());
            }

            for i in 16..64 {
                let s0: u32 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1: u32 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
            }

            let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h)
                = (h0, h1, h2, h3, h4, h5, h6, h7);

            for i in 0..64 {
                let s1: u32 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let s0: u32 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);

                let ch: u32 = (e & f) ^ ((!e) & g);
                let maj: u32 = (a & b) ^ (a & c) ^ (b & c);
                let temp1: u32 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
                let temp2: u32 = s0.wrapping_add(maj);

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }

            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
            h5 = h5.wrapping_add(f);
            h6 = h6.wrapping_add(g);
            h7 = h7.wrapping_add(h);
        }

        format!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4, h5, h6, h7)
    }

    #[cfg(test)]
    mod tests {
        use super::sha256;

        #[test]
        pub fn test_sha_empty_str() {
            let data = "".bytes().collect::<Vec<u8>>();
            assert_eq!(sha256(data),
                       "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        }

        #[test]
        pub fn test_sha_some_text() {
            let data = "abc".bytes().collect::<Vec<u8>>();
            assert_eq!(sha256(data),
                       "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        }

        #[test]
        pub fn test_sha_some_more_text() {
            let data = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".bytes().collect::<Vec<u8>>();
            assert_eq!(sha256(data),
                       "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1")
        }

    }
}