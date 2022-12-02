use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in buffered.lines() {
      if let Ok(content) = line {
        if !content.is_empty() {
          sum+=  content.parse::<i32>().unwrap();
        } else {
          sums.push(sum);
          sum = 0;
        }
      }
    }
    if let Some(max) = sums.iter().max() {
      println!("{}", max);
    }

    Ok(())
}