use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = get_lines("input");

    let move_scores: HashMap<&str, i32> = [("A", 1), ("B", 2), ("C", 3)].into_iter().collect();
    let losing_moves: HashMap<&str, &str> =
        [("A", "C"), ("B", "A"), ("C", "B")].into_iter().collect();
    let winning_moves: HashMap<&str, &str> =
        [("A", "B"), ("B", "C"), ("C", "A")].into_iter().collect();

    let win_score = 6;
    let draw_score = 3;
    let mut sum = 0;

    for line in lines {
        let moves: Vec<&str> = line.split(" ").collect();
        print!("{:?}", moves);
        match moves[1] {
            "X" => {
                let strategic_move = losing_moves.get(moves[0]).unwrap();
                sum += move_scores.get(strategic_move).unwrap();
                println!("lose {:?} {}", strategic_move, sum);
            }
            "Y" => {
                sum += move_scores.get(moves[0]).unwrap() + draw_score;
                println!("draw {:?}", sum);
            }
            "Z" => {
                let strategic_move = winning_moves.get(moves[0]).unwrap();
                sum += move_scores.get(strategic_move).unwrap() + win_score;
                println!("win {:?} {}", strategic_move, sum);
            }
            &_ => {
                panic!("unexpected");
            }
        }
    }
    println!("{}", sum)
}

fn get_lines(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).expect("file not found"))
        .lines()
        .map(|l| l.expect("error line read"))
        .collect()
}
