use std::collections::HashSet;

type Answers = Vec<HashSet<char>>;

#[aoc_generator(day6)]
pub fn generate_input(input: &str) -> Vec<Answers> {
  input
    .split("\n\n")
    .map(|answers| answers.split("\n").map(|a| a.chars().collect()).collect())
    .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(answers: &[Answers]) -> usize {
  count_answers(answers, |acc, p_answers| {
    acc.union(p_answers).cloned().collect()
  })
}

#[aoc(day6, part2)]
pub fn solve_part2(answers: &[Answers]) -> usize {
  count_answers(answers, |acc, p_answers| {
    acc.intersection(p_answers).cloned().collect()
  })
}

fn count_answers<F>(answers: &[Answers], combinator: F) -> usize
where
  F: Fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
{
  answers
    .iter()
    .map(|answers| {
      let init: HashSet<char> = answers.first().unwrap().clone();
      answers
        .iter()
        .skip(1)
        .fold(init, |acc, p_answers| combinator(&acc, p_answers))
        .len()
    })
    .sum()
}
