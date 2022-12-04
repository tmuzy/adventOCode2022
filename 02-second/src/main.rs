use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone)]
struct WinValue {
    score: i32,
    value: i32,
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let scores: HashMap<&str, WinValue> = [
        ("X", WinValue { score: 1, value: 1 }),
        ("Y", WinValue { score: 2, value: 2 }),
        ("Z", WinValue { score: 3, value: 3 }),
        ("A", WinValue { score: 1, value: 2 }),
        ("B", WinValue { score: 2, value: 3 }),
        ("C", WinValue { score: 3, value: 1 }),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum = 0;
    for line in buffered.lines() {
        if let Ok(content) = line {
            if !content.is_empty() {
                let result = content
                    .split(" ")
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .into_boxed_slice();

                sum += scores[result[1]].score;
                if scores[result[1]].value == scores[result[0]].value {
                    println!("win {:?}", result);
                    sum += 6
                } else if scores[result[1]].score == scores[result[0]].score {
                    println!("draw {:?}", result);
                    sum += 3
                } else {
                    println!("lose {:?}", result);
                }
            }
        }
    }
    println!("result {}", sum);
    Ok(())
}
