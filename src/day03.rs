#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
  Tree,
  Free,
}

pub struct TreeMap {
  data: Vec<Vec<Tile>>,
}

impl TreeMap {
  fn height(&self) -> usize {
    self.data.len()
  }

  fn width(&self) -> usize {
    self.data[0].len()
  }

  fn has_tree_at_position(&self, x: usize, y: usize) -> bool {
    y < self.height() && self.get_tile_at_position(x, y) == Tile::Tree
  }

  fn get_tile_at_position(&self, x: usize, y: usize) -> Tile {
    self.data[y][x % self.width()]
  }

  fn tree_count(&self, slope_x: usize, slope_y: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    while y < self.height() {
      if self.has_tree_at_position(x, y) {
        tree_count += 1;
      }
      x += slope_x;
      y += slope_y;
    }

    tree_count
  }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> TreeMap {
  let map_data = input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => Tile::Tree,
          _ => Tile::Free,
        })
        .collect::<Vec<Tile>>()
    })
    .collect();

  TreeMap { data: map_data }
}

#[aoc(day3, part1)]
pub fn solve_part1(tree_map: &TreeMap) -> usize {
  tree_map.tree_count(3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(tree_map: &TreeMap) -> usize {
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|(slope_x, slope_y)| tree_map.tree_count(*slope_x, *slope_y))
    .fold(1, |acc, tree_count| acc * tree_count)
}
