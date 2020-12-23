use std::collections::HashSet;

#[derive(Debug)]
pub struct Seat {
  code: String,
  row: u8,
  col: u8,
  id: u16,
}

impl Seat {
  fn new(code: String) -> Seat {
    let row = code.chars().take(7).enumerate().fold(0, |acc, (i, x)| {
      acc + (1 << (6 - i)) * if x == 'F' { 0 } else { 1 }
    });

    let col = code
      .chars()
      .rev()
      .take(3)
      .enumerate()
      .fold(0, |acc, (i, x)| {
        acc + (1 << i) * if x == 'L' { 0 } else { 1 }
      });

    let id = (row as u16) * 8 + col as u16;

    Seat { code, row, col, id }
  }
}

#[aoc_generator(day5)]
fn generate_input(input: &str) -> Vec<Seat> {
  input.lines().map(|s| Seat::new(s.to_string())).collect()
}

#[aoc(day5, part1)]
fn solve_part1(seats: &[Seat]) -> u16 {
  seats.iter().map(|s| s.id).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(seats: &[Seat]) -> u16 {
  let min = seats.iter().map(|s| s.id).min().unwrap();
  let max = seats.iter().map(|s| s.id).max().unwrap();

  let id_set = seats.iter().map(|s| s.id).collect::<HashSet<_>>();

  (min..max + 1).find(|id| !id_set.contains(id)).unwrap()
}
