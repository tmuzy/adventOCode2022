use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = get_lines("input");
    println!("rucksack priorities sum : {}", get_prio_sum(&lines));
    println!(
        "rucksack group priorities sum : {}",
        get_group_prio_sum(&lines)
    );
}

fn get_lines(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).expect("file not found"))
        .lines()
        .map(|l| l.expect("error line read"))
        .collect()
}

fn get_common_char(a: &str, b: &str) -> char {
    b.chars().filter(|c| a.contains(*c)).last().unwrap()
}

fn get_group_common_char(group: &[String]) -> char {
    group[0]
        .chars()
        .filter(|c| group[1].contains(*c) && group[2].contains(*c))
        .last()
        .unwrap()
}

fn get_char_priority(c: char) -> u32 {
    let value = c as u32 - 48;
    if value >= 48 {
        value - 48
    } else {
        value + 10
    }
}

fn get_prio_sum(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);
        let c = get_common_char(first, second);
        sum += get_char_priority(c)
    }
    sum
}

fn get_group_prio_sum(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for group in lines.chunks_exact(3) {
        let c = get_group_common_char(group);
        sum += get_char_priority(c)
    }
    sum
}
