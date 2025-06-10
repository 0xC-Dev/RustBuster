use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_wordlist(path_to_wordlist: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path_to_wordlist).expect("no such file");
    
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| {
            let line = line.ok()?.trim().to_string();
            if line.starts_with('#') || line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect()
}
