use regex::Regex;
use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = get_lines("input");
    let re: Regex = Regex::new(r"^(?:(\d+)-(\d+)),(?:(\d+)-(\d+))$").unwrap();
    let mut full_overlap_sum = 0;
    let mut overlap_sum = 0;
    for pair in lines {
        for cap in re.captures_iter(&pair) {
            let [one, two, three, four] = [
                &cap[1].parse::<i32>().unwrap(),
                &cap[2].parse::<i32>().unwrap(),
                &cap[3].parse::<i32>().unwrap(),
                &cap[4].parse::<i32>().unwrap(),
            ];
            let overlap = get_overlap([one, two], [three, four]);
            if overlap <= 0 {
                overlap_sum += 1;
                if overlap <= max(one - two, three - four) {
                    full_overlap_sum += 1;
                }
            }
        }
    }
    println!("full overlap {}", full_overlap_sum);
    println!("partial overlap {}", overlap_sum);
}

fn get_overlap(a: [&i32; 2], b: [&i32; 2]) -> i32 {
    max(a[0], b[0]) - min(a[1], b[1])
}

fn get_lines(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).expect("file not found"))
        .lines()
        .map(|l| l.expect("error line read"))
        .collect()
}
