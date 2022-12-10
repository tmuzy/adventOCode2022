use std::{
  fmt::Display,
  fs::File,
  io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
  Up,
  Left,
  Down,
  Right,
}

fn opposite(direction: &Direction) -> Direction {
  match direction {
    Direction::Left => Direction::Right,
    Direction::Up => Direction::Down,
    Direction::Right => Direction::Left,
    Direction::Down => Direction::Up,
  }
}

#[derive(Debug)]
struct Move {
  direction: Direction,
  distance: isize,
}

struct Point {
  x: isize,
  y: isize,
}
impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({};{})", self.x, self.y)
  }
}
impl Point {
  fn pos_after_move(&self, direction: &Direction) -> Point {
    match direction {
      Direction::Left => Point {
        x: self.x - 1,
        y: self.y,
      },
      Direction::Up => Point {
        x: self.x,
        y: self.y + 1,
      },
      Direction::Right => Point {
        x: self.x + 1,
        y: self.y,
      },
      Direction::Down => Point {
        x: self.x,
        y: self.y - 1,
      },
    }
  }

  fn distance_from(&self, other: &Point) -> usize {
    let dx = (other.x - self.x) as f32;
    let dy = (other.y - self.y) as f32;
    (dx * dx + dy * dy).sqrt().round() as usize
  }
}
impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
impl Eq for Point {}

fn main() {
  let mut head = Point { x: 0, y: 0 };
  let mut tail = Point { x: 0, y: 0 };
  let mut visited: Vec<Point> = vec![Point { x: 0, y: 0 }];
  println!("H:{} T:{} d:{}", head, tail, head.distance_from(&tail));
  for line in get_lines("input") {
    println!("{} ====== ", line);
    let parsed_move = parse_move(&line);
    for _ in 0..parsed_move.distance {
      head = head.pos_after_move(&parsed_move.direction);
      if head.distance_from(&tail) > 1 {
        print!("tail move ");
        tail = head.pos_after_move(&opposite(&parsed_move.direction));
        if !visited.iter().find(|p| p == &&tail).is_some() {
          visited.push(Point {
            x: tail.x,
            y: tail.y,
          });
        }
      }
      println!("H:{} T:{} d:{}", head, tail, head.distance_from(&tail));
    }
  }

  for p in &visited {
    println!("{}", p);
  }
  println!("{}", visited.len());
}

fn parse_move(line: &str) -> Move {
  let mut split = line.split_ascii_whitespace();
  let raw_direction = split.next().unwrap();
  let distance = split.next().unwrap().parse::<isize>().unwrap();
  let direction = match raw_direction {
    "U" => Direction::Up,
    "R" => Direction::Right,
    "D" => Direction::Down,
    "L" => Direction::Left,
    &_ => panic!("direction parse error"),
  };
  Move {
    direction: direction,
    distance: distance,
  }
}

fn get_lines(filename: &str) -> Vec<String> {
  BufReader::new(File::open(filename).expect("file not found"))
    .lines()
    .map(|l| l.expect("error line read"))
    .collect()
}
