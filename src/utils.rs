use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::io;

/// Some executables in this course need to distinguish between Encoding and Decoding modes
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
pub fn is_russian_char(c: char) -> bool {
    let c: u32 = c.into();
    (0x0410 <= c && c < 0x0450) || c == 0x0401 || c == 0x0451
}

/// List all files in directory tree into vector.
pub fn files_in_tree(root: &Path) -> io::Result<Vec<PathBuf>> {

    fn accumulate_files(root: &Path, acc: &mut Vec<PathBuf>) -> io::Result<()> {
        if root.is_file() {
            acc.push(root.to_path_buf());
        } else if root.is_dir() {
            for entry in std::fs::read_dir(root)? {
                accumulate_files(&entry?.path(), acc)?;
            }
        }
        Ok(())
    }

    let mut result = Vec::new();
    accumulate_files(root, &mut result)?;
    Ok(result)
}
