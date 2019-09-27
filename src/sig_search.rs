use std::path::{PathBuf, Path};
use std::io;

use memmap::MmapOptions;
use std::fs::File;

#[derive(Debug)]
pub struct SearchResult {
    pub path: PathBuf,
    pub size: u64,
    pub start: u64,
    pub end: u64,
}

const MMAP_THRESHOLD: u64 = 256 * (1 << 20); // 256 MB

/// Search file for a given signature.
pub fn search_file(path: &Path, data: &[u8]) -> io::Result<Option<SearchResult>> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;

    if metadata.len() < data.len() as u64 {
        return Ok(None);
    }

    let automaton = aho_corasick::AhoCorasick::new_auto_configured(&vec![data]);

    let res = if metadata.len() < MMAP_THRESHOLD {
        // file is reasonably small, load into memory to speed up search
        let contents = std::fs::read(path)?;
        automaton.find(contents)
    } else {
        // file is large, mmap it
        let mapped_file = unsafe { MmapOptions::new().map(&file)? };
        automaton.find(mapped_file)
    };

    match res {
        Some(mtch) => Ok(Some(SearchResult {
            path: path.to_owned(),
            size: metadata.len(),
            start: mtch.start() as u64,
            end: mtch.end() as u64
        })),
        None => Ok(None)
    }
}
