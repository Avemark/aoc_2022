use std::collections::HashSet;
use std::io;

fn main() {
    let index = scan_stdin();
    println!("Index: {}", index);
}

fn scan_stdin() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let marker_length = 14;

    let max = input.len() - 1;
    for i in (marker_length)..max {
        let range = (i - marker_length)..=(i - 1);
        if unique(marker_length,&input[range]) {
            return i
        }
    };

    return 0
}

fn unique(expected_len: usize, slice: &str) -> bool {
    let unique: HashSet<_> = slice.chars().collect();
    unique.len() == expected_len
}
