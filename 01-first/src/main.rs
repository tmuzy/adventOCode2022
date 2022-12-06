use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "input";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in buffered.lines() {
        if let Ok(content) = line {
            if !content.is_empty() {
                sum += content.parse::<i32>().unwrap();
            } else {
                sums.push(sum);
                sum = 0;
            }
        }
    }
    sums.sort();
    sums.reverse();
    let max = &sums[..3];
    println!("{:?}", max.iter().sum::<i32>());

    Ok(())
}
