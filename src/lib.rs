#![feature(test)]
extern crate test;

use std::collections::HashMap;

type StrCounts<'a> = HashMap<&'a str, u64>;

type StrFrequencies<'a> = Vec<(&'a str, f64)>;

/// Builds a sliding window iterator for a given string.
/// Equivalent of s.chars().collect::<Vec<char>>().window(win), but does not allocate.
pub fn slide_string(s: &str, win: usize) -> impl Iterator<Item=&str> {
    s
        .char_indices()
        .flat_map(move |(i, _)| {
            s[i..]
                .char_indices()
                .skip(win - 1)
                .next()
                .map(|(j, c)| &s[i .. i + j + c.len_utf8()])
        })
}

/// Computes counts of characters or character clusters in text.
pub fn counts(text: &str, win: usize) -> (u64, StrCounts) {
    let mut freq = StrCounts::with_capacity(1 << 8);

    let mut total = 0u64;
    for piece in slide_string(text, win) {
        *freq.entry(piece).or_insert(0) += 1;
        total += 1;
    }

    (total, freq)
}

/// Computes frequencies of characters or character clusters in text.
pub fn frequencies(text: &str, win: usize) -> StrFrequencies {
    let (total, freq) = counts(text, win);

    if total == 0 {
        StrFrequencies::new()
    } else {
        let mut freq = freq
            .into_iter()
            .map(|(e, n)| (e, n as f64 / total as f64))
            .collect::<StrFrequencies>();

        freq.sort_unstable_by(
            |(_, x), (_, y)| y.partial_cmp(x).unwrap() // no NaNs possible
        );

        freq
    }
}

/// Detects whether a UTF-8 character belongs to a modern Russian character range.
pub fn is_russian_char(c: char) -> bool {
    let c: u32 = c.into();
    (0x0410 <= c && c < 0x0450) || c == 0x0401 || c == 0x0451
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_slide_string_win_1(b: &mut Bencher) {
        let s = "Ж".repeat(1000000);
        b.iter(|| {
            let mut x = 0u64;
            slide_string(&s, 1).for_each(|_| x += 1);
        })
    }

    #[bench]
    pub fn bench_slide_string_win_2(b: &mut Bencher) {
        let s = "Ж".repeat(1000000);
        b.iter(|| {
            let mut x = 0u64;
            slide_string(&s, 2).for_each(|_| x += 1);
        })
    }

    #[bench]
    pub fn bench_counts_window_1(b: &mut Bencher) {
        match std::fs::read_to_string("book.txt") {
            Ok(text) => b.iter(|| counts(text.as_str(), 1).1.len()),
            Err(_) => eprintln!("Cannot run bench for counts, book.txt is not available")
        }
    }

    #[bench]
    pub fn bench_counts_window_2(b: &mut Bencher) {
        match std::fs::read_to_string("book.txt") {
            Ok(text) => b.iter(|| counts(text.as_str(), 2).1.len()),
            Err(_) => eprintln!("Cannot run bench for counts, book.txt is not available")
        }
    }

}
