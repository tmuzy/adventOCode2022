use std::{
  fs::File,
  io::{BufRead, BufReader},
};

#[derive(Debug)]
enum InstructionType {
  Noop,
  Addx(isize),
}

impl PartialEq for InstructionType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (InstructionType::Noop, InstructionType::Noop) => true,
      (InstructionType::Noop, InstructionType::Addx(_)) => false,
      (InstructionType::Addx(_), InstructionType::Noop) => false,
      (InstructionType::Addx(value), InstructionType::Addx(other_value)) => value == other_value,
    }
  }
}

#[derive(Debug)]
struct Instruction {
  t: InstructionType,
}

fn parse_instruction(line: &str) -> Vec<Instruction> {
  if line.starts_with("addx") {
    let value = line
      .split_whitespace()
      .into_iter()
      .last()
      .unwrap()
      .parse::<isize>()
      .unwrap();

    return vec![
      Instruction {
        t: InstructionType::Noop,
      },
      Instruction {
        t: InstructionType::Addx(value),
      },
    ];
  }
  vec![Instruction {
    t: InstructionType::Noop,
  }]
}

fn first_interrupt(x: &isize, cycles: &isize) -> isize {
  if cycles % 40 == 20 {
    return x * cycles;
  }
  0
}

fn second_interrupt(x: &isize, row: &isize) {
  if row == x || row == &(x - 1) || row == &(x + 1) {
    print!("#");
  } else {
    print!(".");
  }
}

fn main() {
  let mut cycles: isize = 0;
  let mut x: isize = 1;
  let mut signal_sums = 0;
  let mut row_i: isize = 0;
  for line in get_lines("input") {
    for instruction in parse_instruction(&line) {
      if let InstructionType::Addx(value) = instruction.t {
        cycles += 1;

        signal_sums += first_interrupt(&x, &cycles);
        second_interrupt(&x, &row_i);
        x += value;
      } else {
        cycles += 1;

        signal_sums += first_interrupt(&x, &cycles);
        second_interrupt(&x, &row_i);
      }

      row_i += 1;
      if cycles % 40 == 0 {
        row_i = 0;
        println!("");
      }
    }
  }
  println!("sum {}", signal_sums)
}

fn get_lines(filename: &str) -> Vec<String> {
  BufReader::new(File::open(filename).expect("file not found"))
    .lines()
    .map(|l| l.expect("error line read"))
    .collect()
}
