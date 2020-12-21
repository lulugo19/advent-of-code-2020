use regex::Regex;

pub struct Password {
  a: usize,
  b: usize,
  letter: char,
  password: String,
}

impl Password {
  pub fn is_valid_policy_1(&self) -> bool {
    let count = self.password.chars().filter(|c| *c == self.letter).count();
    return count >= self.a && count <= self.b;
  }

  pub fn is_valid_policy_2(&self) -> bool {
    let chars = self.password.chars().collect::<Vec<_>>();
    (chars[self.a - 1] == self.letter) ^ (chars[self.b - 1] == self.letter)
  }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
  let re = Regex::new(r"^(?P<a>\d+)-(?P<b>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)$").unwrap();

  input
    .lines()
    .map(|line| {
      let captures = re.captures(line).unwrap();
      Password {
        a: captures["a"].parse::<usize>().unwrap(),
        b: captures["b"].parse::<usize>().unwrap(),
        letter: captures["letter"].chars().next().unwrap(),
        password: captures["password"].to_string(),
      }
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(passwords: &[Password]) -> usize {
  passwords.iter().filter(|p| p.is_valid_policy_1()).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(passwords: &[Password]) -> usize {
  passwords.iter().filter(|p| p.is_valid_policy_2()).count()
}
