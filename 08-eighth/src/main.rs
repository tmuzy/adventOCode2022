use colored::*;
use std::{
  fmt::Display,
  fs::File,
  io::{BufRead, BufReader},
};

const MIN_POS: usize = 0;
struct Pos {
  x: usize,
  y: usize,
}
impl Pos {
  fn pos_after_move(&self, direction: &Direction) -> Pos {
    match direction {
      Direction::Left => Pos {
        x: self.x - 1,
        y: self.y,
      },
      Direction::Top => Pos {
        x: self.x,
        y: self.y - 1,
      },
      Direction::Right => Pos {
        x: self.x + 1,
        y: self.y,
      },
      Direction::Bottom => Pos {
        x: self.x,
        y: self.y + 1,
      },
    }
  }
}
impl PartialEq for Pos {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
struct Tree {
  pos: Pos,
  height: usize,
}

#[derive(Debug)]
enum Direction {
  Left,
  Top,
  Right,
  Bottom,
}

fn get_opposite_direction(direction: &Direction) -> Direction {
  match direction {
    Direction::Left => Direction::Right,
    Direction::Top => Direction::Bottom,
    Direction::Right => Direction::Left,
    Direction::Bottom => Direction::Top,
  }
}

impl Display for Tree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "x:{}, y:{}, h:{}", self.pos.x, self.pos.y, self.height)
  }
}

type TreeGrid = Vec<Tree>;

trait TreeGridOperations {
  fn get_width(&self) -> usize;
  fn get_tree(&self, tree_pos: &Pos) -> Option<&Tree>;
  fn has_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> bool;
  fn get_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> Option<&Tree>;
  fn get_neighbour_height(&self, tree_pos: &Pos, direction: &Direction) -> Option<usize>;
  fn is_tree_on_edge(&self, tree_pos: &Pos, direction: &Direction) -> bool;
  fn calculate_scenic_score(&self, tree_pos: &Pos, direction: &Direction) -> usize;
  fn total_scenic_score(&self, tree_pos: &Pos) -> usize;
  fn get_visible_trees(&self);
}

impl TreeGridOperations for TreeGrid {
  fn get_width(&self) -> usize {
    (self.len() as f64).sqrt() as usize - 1
  }
  fn get_tree(&self, tree_pos: &Pos) -> Option<&Tree> {
    self.iter().find(|t| tree_pos == &t.pos)
  }

  fn has_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> bool {
    match direction {
      Direction::Left => tree_pos.x > MIN_POS,
      Direction::Top => tree_pos.y > MIN_POS,
      Direction::Right => tree_pos.x < self.get_width(),
      Direction::Bottom => tree_pos.y < self.get_width(),
    }
  }

  fn get_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> Option<&Tree> {
    if self.has_neighbour(tree_pos, direction) {
      return self.get_tree(&tree_pos.pos_after_move(direction));
    }
    None
  }

  fn get_neighbour_height(&self, tree_pos: &Pos, direction: &Direction) -> Option<usize> {
    if !self.has_neighbour(tree_pos, direction) {
      return None;
    }
    if let Some(neigh) = self.get_neighbour(tree_pos, direction) {
      return Some(neigh.height);
    }
    None
  }

  fn is_tree_on_edge(&self, tree_pos: &Pos, direction: &Direction) -> bool {
    match direction {
      Direction::Left => !self.has_neighbour(tree_pos, direction),
      Direction::Top => !self.has_neighbour(tree_pos, direction),
      Direction::Right => !self.has_neighbour(tree_pos, direction),
      Direction::Bottom => !self.has_neighbour(tree_pos, direction),
    }
  }

  fn get_visible_trees(&self) {
    const ALL_DIRECTIONS: [Direction; 4] = [
      Direction::Left,
      Direction::Top,
      Direction::Right,
      Direction::Bottom,
    ];

    let mut visible_trees: Vec<&Tree> = Vec::new();
    for direction in ALL_DIRECTIONS {
      for tree in self
        .iter()
        .filter(|t| self.is_tree_on_edge(&t.pos, &direction))
      {
        if visible_trees.iter().find(|t| tree.pos == t.pos).is_none() {
          visible_trees.push(tree);
        }
        let mut current: &Tree = tree;
        let mut highest: &Tree = tree;
        loop {
          if let Some(neigh) = self.get_neighbour(&current.pos, &get_opposite_direction(&direction))
          {
            if neigh.height > highest.height {
              if visible_trees.iter().find(|t| neigh.pos == t.pos).is_none() {
                visible_trees.push(neigh);
              }
            }
            current = neigh;
            if current.height > highest.height {
              highest = current;
            }
          } else {
            break;
          }
        }
      }
    }

    for t in self {
      if visible_trees.iter().find(|vt| vt.pos == t.pos).is_some() {
        print!("{}", t.height);
      } else {
        print!("▒");
      }
      if t.pos.x == self.get_width() {
        println!("")
      }
    }
    println!("count {}", visible_trees.len());
  }

  fn calculate_scenic_score(&self, tree_pos: &Pos, direction: &Direction) -> usize {
    let mut score = 0;
    let mut current: &Tree = self.get_tree(tree_pos).unwrap();
    let start: &Tree = self.get_tree(tree_pos).unwrap();
    loop {
      if let Some(neigh) = self.get_neighbour(&current.pos, &direction) {
        score += 1;
        if neigh.height >= start.height {
          break;
        }
        current = neigh;
      } else {
        break;
      }
    }

    score
  }

  fn total_scenic_score(&self, tree_pos: &Pos) -> usize {
    if self.is_tree_on_edge(tree_pos, &Direction::Bottom)
      || self.is_tree_on_edge(tree_pos, &Direction::Left)
      || self.is_tree_on_edge(tree_pos, &Direction::Right)
      || self.is_tree_on_edge(tree_pos, &Direction::Top)
    {
      return 0;
    }

    let total = self.calculate_scenic_score(tree_pos, &Direction::Bottom)
      * self.calculate_scenic_score(tree_pos, &Direction::Left)
      * self.calculate_scenic_score(tree_pos, &Direction::Right)
      * self.calculate_scenic_score(tree_pos, &Direction::Top);
    total
  }
}

const PART_ONE: bool = false;

fn get_height_color(height: &usize, score: &usize) -> ColoredString {
  match score {
    0 => (" ".to_owned() + &height.to_string()).on_blue(),
    1 => (" ".to_owned() + &height.to_string()).on_bright_blue(),
    2 => (" ".to_owned() + &height.to_string()).on_cyan(),
    3 => (" ".to_owned() + &height.to_string()).on_bright_cyan(),
    4..=10 => (" ".to_owned() + &height.to_string()).on_green(),
    11..=50_000 => (" ".to_owned() + &height.to_string()).on_bright_green(),
    50_001..=100_000 => (" ".to_owned() + &height.to_string()).on_yellow(),
    100_001..=200_000 => (" ".to_owned() + &height.to_string()).on_bright_yellow(),
    200_001..=300_000 => (" ".to_owned() + &height.to_string()).on_bright_red(),
    300_001.. => (" ".to_owned() + &height.to_string()).on_red(),
    &_ => (" ".to_owned() + &height.to_string()).white(),
  }
}

fn main() {
  let mut trees: TreeGrid = Vec::new();
  let lines = get_lines("input");
  for (y, line) in lines.iter().enumerate() {
    for (x, height) in line
      .chars()
      .map(|c| c.to_digit(10).unwrap() as usize)
      .enumerate()
    {
      let tree = Tree {
        pos: Pos { x: x, y: y },
        height: height,
      };
      trees.push(tree)
    }
  }

  if PART_ONE {
    trees.get_visible_trees();
  }

  let scores: Vec<(&Tree, usize)> = trees
    .iter()
    .map(|t| (t, trees.total_scenic_score(&t.pos)))
    .collect();
  let best = scores.iter().max_by_key(|t| t.1).unwrap().1;
  for t in scores {
    print!("{}", get_height_color(&t.0.height, &t.1));

    if t.0.pos.x == trees.get_width() {
      println!("")
    }
  }
  println!("\r");
  println!("best scenic score {}", best);
}

fn get_lines(filename: &str) -> Vec<String> {
  BufReader::new(File::open(filename).expect("file not found"))
    .lines()
    .map(|l| l.expect("error line read"))
    .collect()
}
