use std::collections::HashMap;

type StrCounts<'a> = HashMap<&'a str, u64>;

type StrFrequencies<'a> = Vec<(&'a str, f64)>;

/// Builds a sliding window iterator for a given string.
/// Equivalent of s.chars().collect::<Vec<char>>().window(win), but does not allocate.
pub fn slide_string(s: &str, win: usize) -> impl Iterator<Item=&str> {
    s
        .char_indices()
        .zip(s.char_indices().skip(win - 1))
        .map(move |((i, _), (j, c))| &s[i .. j + c.len_utf8()])
}

/// Computes counts of characters or character clusters in text, with respect to filter.
/// Filter is applied as following: if all chars in window satisfy filter, then keep this window,
/// else skip it (this helps to respect word boundaries, for example).
pub fn counts<F>(text: &str, filter: F, win: usize) -> (u64, StrCounts)
    where F: Fn(char,) -> bool
{
    let mut freq = StrCounts::with_capacity(1 << 8);

    let mut total = 0u64;
    for piece in slide_string(text, win) {
        if piece.chars().all(&filter) {
            *freq.entry(piece).or_insert(0) += 1;
            total += 1;
        }
    }

    (total, freq)
}

/// Computes frequencies of characters or character clusters in text.
pub fn frequencies<F>(text: &str, filter: F, win: usize) -> StrFrequencies
    where F: Fn(char,) -> bool
{
    let (total, freq) = counts(text, filter, win);

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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_slide_string_win_1_1mb_text(b: &mut Bencher) {
        let s = "Ж".repeat(1 << 20);
        b.iter(|| {
            let mut x = 0u64;
            slide_string(&s, 1).for_each(|_| x += 1);
        })
    }

    #[bench]
    pub fn bench_slide_string_win_2_256chars_text(b: &mut Bencher) {
        let s = "a".repeat(1 << 8);
        b.iter(|| {
            let mut x = 0u64;
            slide_string(&s, 2).for_each(|_| x += 1);
        })
    }

    #[bench]
    pub fn bench_slide_string_win_2_1mb_text(b: &mut Bencher) {
        let s = "Ж".repeat(1 << 20);
        b.iter(|| {
            let mut x = 0u64;
            slide_string(&s, 2).for_each(|_| x += 1);
        })
    }

}
