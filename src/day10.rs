#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u32> {
  let mut adapters: Vec<u32> = input
    .lines()
    .map(|line| line.parse::<u32>().unwrap())
    .collect();
  adapters.sort();
  adapters
}

#[aoc(day10, part1)]
// the adapters should already be sorted
pub fn solve_day1(adapters: &[u32]) -> u32 {
  let difference_count = get_jolts_difference_count(adapters);
  difference_count[0] * difference_count[2]
}

pub fn get_jolts_difference_count(adapters: &[u32]) -> [u32; 3] {
  let mut difference_count: [u32; 3] = [0, 0, 1];

  let mut last_adapter = 0;
  for adapter in adapters {
    let diff = adapter - last_adapter;
    if diff > 3 {
      panic!("To high jolts difference: {}", diff);
    } else if diff == 0 {
      panic!("To low jolts difference!")
    }
    difference_count[diff as usize - 1] += 1;
    last_adapter = *adapter;
  }

  difference_count
}

#[aoc(day10, part2)]
// the adapters should already be sorted
pub fn solve_day2(adapters: &[u32]) -> u64 {
  let mut arrangements: Vec<u64> = Vec::new();

  for i in 0..adapters.len() as i32 {
    let mut next_arrangements = 0;
    for j in 1..=3 as i32 {
      let k = i - j;

      if k >= 0 {
        if adapters[i as usize] - adapters[k as usize] <= 3 {
          next_arrangements += arrangements[k as usize];
        }
      } else if k == -1 {
        // handling the jump from 0 to adapters[i] because zero is not included in the adapters
        if adapters[i as usize] <= 3 {
          next_arrangements += 1;
        }
      }
    }

    arrangements.push(next_arrangements);
  }
  *arrangements.last().unwrap()
}

#[cfg(test)]
mod tests {
  use super::get_jolts_difference_count;
  use super::solve_day2;

  lazy_static! {
    static ref TEST_INPUT_SMALL: Vec<u32> = {
      let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
      input.sort();
      input
    };
    static ref TEST_INPUT_LARGER: Vec<u32> = {
      let mut input = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
      ];
      input.sort();
      input
    };
  }

  fn test_solve_day1(adapters: &Vec<u32>, expected: (u32, u32)) {
    let diff_count = get_jolts_difference_count(&adapters);
    assert_eq!(diff_count[0], expected.0);
    assert_eq!(diff_count[2], expected.1);
  }

  fn test_solve_day_2(adapters: &Vec<u32>, expected: u64) {
    assert_eq!(solve_day2(&adapters), expected);
  }

  #[test]
  fn test_day10_part1_small() {
    test_solve_day1(&TEST_INPUT_SMALL, (7, 5));
  }

  #[test]
  fn test_day10_part1_larger() {
    test_solve_day1(&TEST_INPUT_LARGER, (22, 10));
  }

  #[test]
  fn test_day10_part2_small() {
    test_solve_day_2(&TEST_INPUT_SMALL, 8);
  }

  #[test]
  fn test_day10_part2_larger() {
    test_solve_day_2(&TEST_INPUT_LARGER, 19208);
  }
}
