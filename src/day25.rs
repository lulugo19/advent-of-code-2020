#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (u64, u64) {
  let mut lines = input.lines().map(|line| line.parse().unwrap());

  (lines.next().unwrap(), lines.next().unwrap())
}

#[aoc(day25, part1)]
pub fn solve_part1((card_public_key, door_public_key): &(u64, u64)) -> u64 {
  let card_loop_size = reverse_engineer_loop_size(7, *card_public_key);
  let encryption_key = handshake_operation(*door_public_key, card_loop_size);
  return encryption_key;
}

fn reverse_engineer_loop_size(subject_number: u64, public_key: u64) -> u64 {
  let mut value = 1;
  let mut loop_size = 0;
  loop {
    if value == public_key {
      return loop_size;
    }
    value = (value * subject_number) % 20201227;
    loop_size += 1;
  }
}

fn handshake_operation(subject_number: u64, loop_size: u64) -> u64 {
  let mut value = 1;
  for _ in 0..loop_size {
    value = (value * subject_number) % 20201227;
  }
  return value;
}

#[cfg(test)]
mod test {
  use super::solve_part1;

  #[test]
  fn test_day25_part1() {
    assert_eq!(solve_part1(&(5764801, 17807724)), 14897079)
  }
}
