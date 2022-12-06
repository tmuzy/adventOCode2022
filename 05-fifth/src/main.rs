use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct ElfCrate {
    label: char,
}

type ElfCrateStack = Vec<ElfCrate>;

type ElfCrates = [ElfCrateStack; 9];
trait CrateOperations {
    fn count_crates(&mut self, x: usize) -> usize;

    fn move_crate(&mut self, from_stack_index: usize, to_stack_index: usize);
    fn move_crates(&mut self, amount: usize, from_stack_index: usize, to_stack_index: usize);

}

impl CrateOperations for ElfCrates {
    fn count_crates(&mut self, x: usize) -> usize {
        self[x].iter_mut().count()
    }

    fn move_crate(&mut self, from_stack_index: usize, to_stack_index: usize) {
        let popped = self[from_stack_index].pop().unwrap();
        self[to_stack_index].push(popped);
    }

    fn move_crates(&mut self, amount: usize, from_stack_index: usize, to_stack_index: usize) {
        let from_stack = &mut self[from_stack_index];
        let drained:Vec<ElfCrate> = from_stack.drain(from_stack.len()-amount..).collect();
        self[to_stack_index].extend(drained);
    }
}
fn main() {
    let lines = get_lines("input");

    let crates_moves_separation = lines.iter().position(|line| line.is_empty()).unwrap();
    let crates_and_moves = lines.split_at(crates_moves_separation);

    let (crates, moves) = crates_and_moves;
    let mut elf_crates: ElfCrates = parse_crates(crates);

    let moves = parse_moves(moves);
    // part 1 -- too lazy to do both at the same time
    // for crate_move in moves {
    //     for _ in 0..crate_move.0 {
    //         elf_crates.move_crate(crate_move.1 - 1, crate_move.2 - 1)
    //     }
    // }

    for crate_move in moves {
        elf_crates.move_crates(crate_move.0, crate_move.1 - 1, crate_move.2 - 1)
    }


    println!("{}", elf_crates.map(|ec| ec.last().unwrap().label.to_string()).join(""))
}

/** in `[1; 9]` */
fn get_crate_index(index: usize) -> usize {
    (index as f32 / 4.0).ceil() as usize
}

fn parse_crates(crates: &[String]) -> ElfCrates {
    let crate_name_search: Regex = Regex::new(r"^[^\s\[\]\d]$").unwrap();
    let mut elf_crates: [ElfCrateStack; 9] = Default::default();
    for (_, line) in crates.iter().rev().enumerate() {
        for unit in line
            .char_indices()
            .filter(|(_, c)| crate_name_search.is_match(&c.to_string()))
        {
            let x: usize = get_crate_index(unit.0) - 1;

            elf_crates[x].push(ElfCrate { label: unit.1 });
        }
    }
    elf_crates
}

fn parse_moves(moves: &[String]) -> Vec<(usize, usize, usize)> {
    let re: Regex = Regex::new(r"\d+").unwrap();
    let mut crate_moves: Vec<(usize, usize, usize)> = Default::default();
    for line in moves[1..].iter() {
        let find = &re
            .find_iter(line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect::<Vec<usize>>();
        crate_moves.push((find[0], find[1], find[2]));
    }
    crate_moves
}

fn get_lines(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).expect("file not found"))
        .lines()
        .map(|l| l.expect("error line read"))
        .collect()
}
