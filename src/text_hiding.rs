use std::iter::Iterator;

/// Returns an iterator over bits (as bool values) in string.
pub fn iterate_bits(s: &str) -> impl Iterator<Item=bool> + '_ {
    s
        .bytes()
        .flat_map(
            move |b|
                (0..8).map(
                    move |i| (b & (1 << i) != 0)
                )
        )
}

/// Accumulates bits from bool iterator into u8 buffer
pub fn accumulate_bits<Iter>(it: Iter, buf: &mut Vec<u8>)
    where Iter: Iterator<Item=bool>
{
    const BITS_IN_BYTE: u8 = 8;

    let mut cur = 0u8;
    let mut cur_idx = 0;

    for bit in it {
        if bit {
            cur |= 1 << cur_idx;
        }
        cur_idx += 1;

        if cur_idx == BITS_IN_BYTE {
            buf.push(cur);
            cur = 0u8;
            cur_idx = 0;
        }
    }

    if cur_idx != 0 {
        buf.push(cur);
    }
}

/// Returns the length of a string in bits
pub fn len_bits(s: &str) -> usize {
    s.len() * 8usize
}

/// Remove trailing zeros from vector of bytes.
pub fn remove_trailing_zeros(buf: &mut Vec<u8>) {
    while buf.last().map_or(false, |&b| b == 0u8) {
        buf.pop();
    }
}


#[derive(Debug)]
pub struct AlgorithmError { pub what: String }


pub mod lines {
    use super::*;

    /// Hide text in the multi-line container
    pub fn hide(text: &str, container: &mut Vec<String>, secret_char: char) -> Result<String, AlgorithmError> {
        if len_bits(text) > container.len() {
            return Err(AlgorithmError {
                what: format!("Cannot hide text of length {} bits in {}-line container",
                              len_bits(text), container.len()
                )
            });
        }

        iterate_bits(text).enumerate().for_each(
            |(i, bit)| if bit { container[i].push(secret_char) }
        );

        Ok(container.join("\n"))
    }

    /// Reveal text hidden in multi-line container
    pub fn reveal(container: &Vec<String>, secret_char: char) -> Result<String, AlgorithmError> {
        let mut buf = Vec::with_capacity(container.len());

        accumulate_bits(
            container
                .iter()
                .map(|s| s.chars().rev().next().map_or(false, |c| c == secret_char)),
            &mut buf
        );

        remove_trailing_zeros(&mut buf);

        String::from_utf8(buf).map_err(|_| AlgorithmError { what: "Failed to decode UTF8".to_string() })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn test_hide_reveal() {
            let text = "ыs";
            let mut container = "line\n"
                .repeat(len_bits(text) + 1)
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let secret_char = ' ';

            assert!(hide(text, &mut container, secret_char).is_ok());

            for (line, bit) in container.iter().zip(iterate_bits(text)) {
                assert!(!bit || line.ends_with(secret_char))
            }

            let decoded = reveal(&container, secret_char);

            assert!(decoded.is_ok());
            assert_eq!(decoded.unwrap(), text);
        }
    }
}


pub mod ru_en_similarity {
    use super::*;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[derive(Copy, Clone)]
    pub enum MappingDirection {
        RuEn, EnRu
    }

    impl MappingDirection {

        pub fn invert(&self) -> Self {
            match self {
                MappingDirection::RuEn => MappingDirection::EnRu,
                MappingDirection::EnRu => MappingDirection::RuEn,
            }
        }

    }

    impl FromStr for MappingDirection {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "ru_en" | "RuEn" => Ok(MappingDirection::RuEn),
                "en_ru" | "EnRu" => Ok(MappingDirection::EnRu),
                _ => Err(())
            }
        }
    }

    // Not present as a static variable because there are constraints on compile-time code
    fn create_char_map(dir: MappingDirection) -> HashMap<char, char> {
        let mut map = Vec::new();

        map.push(('a', 'а'));
        map.push(('c', 'с'));
        map.push(('e', 'е'));
        map.push(('o', 'о'));
        map.push(('p', 'р'));
        map.push(('y', 'у'));

        map.push(('A', 'А'));
        map.push(('B', 'В'));
        map.push(('C', 'С'));
        map.push(('E', 'Е'));
        map.push(('H', 'Н'));
        map.push(('K', 'К'));
        map.push(('M', 'М'));
        map.push(('O', 'О'));
        map.push(('P', 'Р'));
        map.push(('T', 'Т'));
        map.push(('X', 'Х'));

        match dir {
            MappingDirection::EnRu => map.into_iter().collect(),
            MappingDirection::RuEn => map.into_iter().map(|(c1, c2)| (c2, c1)).collect(),
        }
    }

    /// Hide text in the container
    pub fn hide(text: &str, container: &str, direction: MappingDirection) -> Result<String, AlgorithmError> {
        let map = create_char_map(direction);
        let mut bits = iterate_bits(text);

        let result = container
            .chars()
            .map(|c|
                map
                    .get(&c)
                    .map_or(c, |cc| if bits.next().unwrap_or(false) { *cc } else { c })
            )
            .collect::<String>();

        if bits.next().is_none() {
            Ok(result)
        } else {
            Err(AlgorithmError {
                what: "Container is too small to hide given information".to_string()
            })
        }
    }

    /// Reveal text hidden in the container
    pub fn reveal(container: &str, direction: MappingDirection) -> Result<String, AlgorithmError> {
        let map = create_char_map(direction);
        let inverse_map = create_char_map(direction.invert());

        let result = container
            .chars()
            .filter_map(|c| {
                if map.contains_key(&c) {
                    Some(false)
                } else if inverse_map.contains_key(&c) {
                    Some(true)
                } else {
                    None
                }
            });

        let mut buf = Vec::new();
        accumulate_bits(result, &mut buf);

        remove_trailing_zeros(&mut buf);

        String::from_utf8(buf).map_err(|_| AlgorithmError {
            what: "Failed to decode text".to_string()
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn test_hide_reveal() {
            let text = "ыs";
            let container = "a".repeat(len_bits(text) + 1);

            let container = hide(text, &container, MappingDirection::EnRu);
            assert!(container.is_ok());

            let decoded = reveal(&container.unwrap(), MappingDirection::EnRu);

            assert!(decoded.is_ok());
            assert_eq!(decoded.unwrap(), text);
        }
    }
}
