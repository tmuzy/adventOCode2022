use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let mut sum = 0;
    for l in get_lines("input") {
        if let Ok(line) = l {
            let (first, second) = line.split_at(line.len() / 2);
            println!("{} {}", first, second);
            let c = get_common_char(first, second);
            println!("{} {}", c, get_char_priority(c));
            sum += get_char_priority(c)
        }
    }
    println!("{}", sum)
}

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    let input = File::open(path).expect("file not found");
    let buff = BufReader::new(input);
    buff.lines()
}

fn get_common_char(a: &str, b: &str) -> char {
    b.chars().filter(|c| a.contains(*c)).last().unwrap()
}

fn get_char_priority(c: char) -> u32 {
    let value = c as u32 - 48;
    if value >= 48 {
        value - 48
    } else {
        value + 10
    }
}
