use std::{fs, collections::HashSet};


fn main() {
    let window_size:usize = 14; //4 for part one, 14 for part two
    let file_content = get_content("input");
    let char_set = file_content.chars()
    .collect::<Vec<char>>()
    .windows(window_size)
    .map(|c| c.iter().collect::<String>())
    .enumerate()
    .filter_map(|(i, value)| all_unique_chars(i,value))
    .collect::<Vec<(usize, String)>>();
    println!("{}", char_set.first().unwrap().0 + window_size)
}

fn all_unique_chars(index: usize, value: String) -> Option<(usize, String)> {
    let set: HashSet<char> = HashSet::from_iter(value.chars().collect::<Vec<char>>());
    if set.len() == value.len() {
        Some((index, value))
    } else {
        None
    }
}

fn get_content(filename: &str) -> String {
    fs::read_to_string(filename).expect("no file")
}
