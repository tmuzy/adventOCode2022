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

struct Directions {
  left: bool,
  right: bool,
  top: bool,
  bottom: bool,
}

impl Display for Directions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "←: {}, ↑:{}, →{}, ↓:{}",
      self.left, self.top, self.right, self.bottom
    )
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
  fn has_left_neighbour(&self, tree_pos: &Pos) -> bool;
  fn has_right_neighbour(&self, tree_pos: &Pos) -> bool;
  fn has_top_neighbour(&self, tree_pos: &Pos) -> bool;
  fn has_bottom_neighbour(&self, tree_pos: &Pos) -> bool;
  fn has_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> bool;
  fn get_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> Option<&Tree>;
  fn get_neighbour_height(&self, tree_pos: &Pos, direction: &Direction) -> Option<usize>;
  fn get_height(&self, tree_pos: &Pos) -> Option<usize>;

  fn is_tree_visible_from(&self, tree_pos: &Pos, direction: &Direction) -> bool;
  fn is_tree_on_edge(&self, tree_pos: &Pos, direction: &Direction) -> bool;
  fn is_tree_in_corner(&self, tree_pos: &Pos) -> bool;
}

impl TreeGridOperations for TreeGrid {
  fn get_width(&self) -> usize {
    (self.len() as f64).sqrt() as usize - 1
  }
  fn get_tree(&self, tree_pos: &Pos) -> Option<&Tree> {
    self.iter().find(|t| tree_pos == &t.pos)
  }
  fn get_height(&self, tree_pos: &Pos) -> Option<usize> {
    if let Some(tree) = self.get_tree(tree_pos) {
      return Some(tree.height);
    }
    None
  }
  fn has_left_neighbour(&self, tree_pos: &Pos) -> bool {
    tree_pos.x > MIN_POS
  }
  fn has_right_neighbour(&self, tree_pos: &Pos) -> bool {
    tree_pos.x < self.get_width()
  }
  fn has_top_neighbour(&self, tree_pos: &Pos) -> bool {
    tree_pos.y > MIN_POS
  }
  fn has_bottom_neighbour(&self, tree_pos: &Pos) -> bool {
    tree_pos.y < self.get_width()
  }

  fn has_neighbour(&self, tree_pos: &Pos, direction: &Direction) -> bool {
    match direction {
      Direction::Left => self.has_left_neighbour(tree_pos),
      Direction::Top => self.has_top_neighbour(tree_pos),
      Direction::Right => self.has_right_neighbour(tree_pos),
      Direction::Bottom => self.has_bottom_neighbour(tree_pos),
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

  fn is_tree_visible_from(&self, tree_pos: &Pos, direction: &Direction) -> bool {
    self.get_height(tree_pos) > self.get_neighbour_height(tree_pos, direction)
  }

  fn is_tree_on_edge(&self, tree_pos: &Pos, direction: &Direction) -> bool {
    match direction {
      Direction::Left => !self.has_left_neighbour(tree_pos),
      Direction::Top => !self.has_top_neighbour(tree_pos),
      Direction::Right => !self.has_right_neighbour(tree_pos),
      Direction::Bottom => !self.has_bottom_neighbour(tree_pos),
    }
  }

  fn is_tree_in_corner(&self, tree_pos: &Pos) -> bool {
    (self.is_tree_on_edge(tree_pos, &Direction::Left)
      && self.is_tree_on_edge(tree_pos, &Direction::Top))
      || (self.is_tree_on_edge(tree_pos, &Direction::Right)
        && self.is_tree_on_edge(tree_pos, &Direction::Top))
      || (self.is_tree_on_edge(tree_pos, &Direction::Bottom)
        && self.is_tree_on_edge(tree_pos, &Direction::Left))
      || (self.is_tree_on_edge(tree_pos, &Direction::Bottom)
        && self.is_tree_on_edge(tree_pos, &Direction::Right))
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

  const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Top,
    Direction::Right,
    Direction::Bottom,
  ];

  let mut visible_trees: Vec<&Tree> = Vec::new();
  for direction in ALL_DIRECTIONS {
    for tree in trees
      .iter()
      .filter(|t| trees.is_tree_on_edge(&t.pos, &direction))
    {
      if visible_trees.iter().find(|t| tree.pos == t.pos).is_none() {
        visible_trees.push(tree);
      }
      let mut current: &Tree = tree;
      let mut highest: &Tree = tree;
      loop {
        if let Some(neigh) = trees.get_neighbour(&current.pos, &get_opposite_direction(&direction))
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

  for t in &trees {
    if visible_trees.iter().find(|vt| vt.pos == t.pos).is_some() {
      print!("{}", t.height);
    } else {
      print!("▒");
    }
    if t.pos.x == trees.get_width() {
      println!("")
    }
  }
  println!("count {}", visible_trees.len())
}

fn get_lines(filename: &str) -> Vec<String> {
  BufReader::new(File::open(filename).expect("file not found"))
    .lines()
    .map(|l| l.expect("error line read"))
    .collect()
}
