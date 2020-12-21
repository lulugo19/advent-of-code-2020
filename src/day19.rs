use std::collections::HashMap;

type TerminalRule = (usize, char);
type ProductionRule = (usize, usize, usize);

#[derive(Debug)]
pub struct Input {
  non_terminal_count: usize,
  terminal_rules: Vec<TerminalRule>,
  production_rules: Vec<ProductionRule>,
  words: Vec<String>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
  let mut blocks = input.split("\n\n");

  let mut terminal_rules: Vec<TerminalRule> = Vec::new();
  let mut production_rules: Vec<ProductionRule> = Vec::new();

  let mut non_terminal_count = 0;

  let mut unit_rules: HashMap<usize, Vec<usize>> = HashMap::new();

  blocks.next().unwrap().lines().for_each(|rule| {
    let mut left_right = rule.split(": ");
    let left = left_right.next().unwrap().parse::<usize>().unwrap();
    non_terminal_count = usize::max(non_terminal_count, left);
    let right = left_right.next().unwrap();
    if right.starts_with("\"") {
      terminal_rules.push((left, right.chars().nth(1).unwrap()));
    } else {
      right.split(" | ").for_each(|rule| {
        let non_terminals = rule
          .split(" ")
          .map(|x| x.parse::<usize>().unwrap())
          .collect::<Vec<_>>();
        match non_terminals.len() {
          1 => {
            unit_rules
              .entry(left)
              .or_insert(Vec::new())
              .push(non_terminals[0]);
          }
          2 => {
            production_rules.push((left, non_terminals[0], non_terminals[1]));
          }
          _ => unreachable!(),
        }
      });
    }
  });

  non_terminal_count += 1;

  let mut additional_rules: Vec<ProductionRule> = Vec::new();

  // replace unit rules
  production_rules.iter_mut().for_each(|rule| {
    if unit_rules.contains_key(&rule.1) && unit_rules.contains_key(&rule.2) {
      let values_1 = unit_rules.get(&rule.1).unwrap();
      let values_2 = unit_rules.get(&rule.2).unwrap();

      rule.1 = values_1[0];
      rule.2 = values_2[0];

      use itertools::Itertools;

      additional_rules.extend(
        values_1
          .iter()
          .cartesian_product(values_2.iter())
          .skip(1)
          .map(|(first, second)| (rule.0, *first, *second)),
      );
    } else if unit_rules.contains_key(&rule.1) {
      let values = unit_rules.get(&rule.1).unwrap();
      rule.1 = values[0];
      additional_rules.extend(values.iter().skip(1).map(|first| (rule.0, *first, rule.2)));
    } else if unit_rules.contains_key(&rule.2) {
      let values = unit_rules.get(&rule.2).unwrap();
      rule.2 = values[0];
      additional_rules.extend(
        values
          .iter()
          .skip(1)
          .map(|second| (rule.0, rule.1, *second)),
      );
    }
  });

  production_rules.extend(additional_rules);

  let words = blocks
    .next()
    .unwrap()
    .lines()
    .map(|x| x.to_string())
    .collect();

  Input {
    non_terminal_count,
    terminal_rules,
    production_rules,
    words,
  }
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> usize {
  count_words_in_language(
    &input.words,
    input.non_terminal_count,
    &input.production_rules,
    &input.terminal_rules,
  )
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> usize {
  let mut extended_production_rules = input.production_rules.clone();

  extended_production_rules.push((42, 42, 42));

  let mut non_terminal_count = input.non_terminal_count;

  extended_production_rules.push((11, 42, non_terminal_count));
  extended_production_rules.push((non_terminal_count, 11, 31));

  non_terminal_count += 1;

  count_words_in_language(
    &input.words,
    non_terminal_count,
    &extended_production_rules,
    &input.terminal_rules,
  )
}

fn count_words_in_language(
  words: &[String],
  non_terminal_count: usize,
  production_rules: &[ProductionRule],
  terminal_rules: &[TerminalRule],
) -> usize {
  words
    .iter()
    .filter(|w| cyk(w, non_terminal_count, production_rules, terminal_rules))
    .count()
}

// from https://en.wikipedia.org/wiki/CYK_algorithm#Algorithm
fn cyk(
  word: &str,
  non_terminal_count: usize,
  production_rules: &[ProductionRule],
  terminal_rules: &[TerminalRule],
) -> bool {
  let chars: Vec<char> = word.chars().collect();
  let n = word.len();
  let r = non_terminal_count;

  let mut matrix = vec![vec![vec![false; r]; n]; n];

  for s in 0..n {
    let terminal = chars[s];
    let left = terminal_rules
      .iter()
      .find(|(_, t)| *t == terminal)
      .unwrap()
      .0;
    matrix[0][s][left] = true;
  }

  for l in 1..n {
    for s in 0..n - l {
      for p in 0..l {
        for (a, b, c) in production_rules.iter() {
          if matrix[p][s][*b] && matrix[l - p - 1][s + p + 1][*c] {
            matrix[l][s][*a] = true;
          }
        }
      }
    }
  }

  let is_member_of_language = matrix[n - 1][0][0];

  /*
  if is_member_of_language {
    println!("word '{}' is a member of the language!", word);
  } else {
    println!("word '{}' isn't a member of the language!", word);
  }*/

  return is_member_of_language;
}

#[cfg(test)]
mod test {

  use super::{input_generator, solve_part1};

  const TEST_INPUT: &'static str = "0: 5 1\n\
  1: 2 6\n\
  2: 3 4 | 4 3\n\
  3: 5 5 | 6 6\n\
  4: 5 6 | 6 5\n\
  5: \"a\"\n\
  6: \"b\"\n\
  \n\
  ababbb\n\
  bababa\n\
  abbbab\n\
  aaabbb\n\
  aaaabbb";

  #[test]
  fn test_day19_part1() {
    assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 2);
  }
}
