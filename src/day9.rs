use crate::NoSolutionErr;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
  input
    .lines()
    .map(|line| line.parse::<u64>().unwrap())
    .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(numbers: &[u64]) -> Result<u64, NoSolutionErr> {
  find_xmas_weakness(numbers, 25)
}

fn find_xmas_weakness(numbers: &[u64], preample_length: usize) -> Result<u64, NoSolutionErr> {
  let mut i = preample_length;

  while i < numbers.len() {
    let next = numbers[i];
    let j = i - preample_length;

    let mut found_sum = false;

    for x in &numbers[j..j + preample_length - 1] {
      for y in &numbers[j + 1..j + preample_length] {
        if x + y == next {
          found_sum = true;
          break;
        }
      }
      if found_sum {
        break;
      }
    }

    if !found_sum {
      return Ok(next);
    }
    i += 1;
  }
  Err(NoSolutionErr {})
}

#[aoc(day9, part2)]
pub fn solve_part2(numbers: &[u64]) -> Result<u64, NoSolutionErr> {
  let goal = solve_part1(numbers)?;

  let mut start = 0;
  let mut curr_sum = numbers[0];

  for i in 1..numbers.len() {
    while curr_sum > goal && start < i - 1 {
      curr_sum = curr_sum - numbers[start];
      start += 1;
    }

    if curr_sum == goal {
      let range = &numbers[start..i + 1];
      println!("Found Range: sum({:?}) = {:?}", range, goal);
      return Ok(range.iter().min().unwrap() + range.iter().max().unwrap());
    }

    curr_sum += numbers[i];
  }

  Err(NoSolutionErr {})
}
