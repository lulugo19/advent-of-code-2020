use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
  Plus,
  Times,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
  Number(u64),
  LeftParen,
  RightParen,
  Operator(Operator),
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<Token>> {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .filter_map(|c| match c {
          '(' => Some(Token::LeftParen),
          ')' => Some(Token::RightParen),
          '+' => Some(Token::Operator(Operator::Plus)),
          '*' => Some(Token::Operator(Operator::Times)),
          // There are only on-digit numbers, so this simplification is alright
          _ if c.is_numeric() => Some(Token::Number(c.to_string().parse().unwrap())),
          ' ' => None,
          _ => panic!("Invalid character '{}'!", c),
        })
        .collect()
    })
    .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(lines: &[Vec<Token>]) -> u64 {
  evaluate_lines(lines, false)
}

#[aoc(day18, part2)]
pub fn solve_part2(lines: &[Vec<Token>]) -> u64 {
  evaluate_lines(lines, true)
}

fn evaluate_lines(lines: &[Vec<Token>], is_part2: bool) -> u64 {
  lines
    .iter()
    .map(|l| evaluate(&mut l.iter().peekable(), is_part2))
    .sum()
}

fn evaluate<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>, is_part2: bool) -> u64 {
  let mut a = operand(tokens, is_part2);

  loop {
    if let Some(op) = operator(tokens) {
      let b = operand(tokens, is_part2);
      match op {
        Operator::Plus => a += b,
        Operator::Times => a *= b,
      }
    } else {
      break;
    }
  }

  a
}

fn operand<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>, is_part2: bool) -> u64 {
  let a = match tokens.next().unwrap() {
    Token::Number(x) => *x,
    Token::LeftParen => evaluate(tokens, is_part2),
    _ => panic!("Expected operand!"),
  };
  // modification for part2: if there is a plus operator we add the next operand
  if is_part2 && tokens.peek() == Some(&&Token::Operator(Operator::Plus)) {
    tokens.next();
    a + operand(tokens, is_part2)
  } else {
    a
  }
}

fn operator<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Option<&'a Operator> {
  match tokens.next() {
    Some(Token::Operator(op)) => Some(op),
    _ => None,
  }
}

#[cfg(test)]
mod test {
  use super::{input_generator, solve_part2};

  fn test_part2(input: &str, expected: u64) {
    assert_eq!(solve_part2(&input_generator(input)), expected)
  }

  #[test]
  fn test_day18_part2() {
    test_part2("1 + (2 * 3) + (4 * (5 + 6))", 51);
    test_part2("2 * 3 + (4 * 5)", 46);
    test_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445);
    test_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
    test_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);
  }
}
