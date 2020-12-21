use std::collections::HashSet;

type Ticket = Vec<u64>;
type Rule = (String, Vec<Range>);

#[derive(Debug)]
pub struct Input {
  rules: Vec<Rule>,
  my_ticket: Ticket,
  nearby_tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
  min: u64,
  max: u64,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
  let blocks = input.split("\n\n").collect::<Vec<&str>>();

  let rules: Vec<Rule> = blocks[0]
    .lines()
    .map(|line| {
      let key_value = line.split(": ").collect::<Vec<&str>>();
      let ranges = key_value[1]
        .split(" or ")
        .map(|r| {
          let min_max = r.split("-").collect::<Vec<&str>>();
          Range {
            min: min_max[0].parse().unwrap(),
            max: min_max[1].parse().unwrap(),
          }
        })
        .collect();
      (key_value[0].to_string(), ranges)
    })
    .collect();

  let my_ticket: Ticket = blocks[1]
    .split("\n")
    .nth(1)
    .unwrap()
    .split(",")
    .map(|x| x.parse().unwrap())
    .collect();

  let nearby_tickets: Vec<Ticket> = blocks[2]
    .split("\n")
    .skip(1)
    .map(|row| row.split(",").map(|x| x.parse().unwrap()).collect())
    .collect();

  Input {
    rules,
    my_ticket,
    nearby_tickets,
  }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> u64 {
  input
    .nearby_tickets
    .iter()
    .map(|ticket| ticket_error_rate(ticket, &input.rules))
    .sum()
}

fn ticket_error_rate(ticket: &Ticket, rules: &[Rule]) -> u64 {
  ticket
    .iter()
    .filter(|x| !rules.iter().any(|r| rule_is_valid_for_field(&r.1, **x)))
    .sum()
}

fn ticket_is_valid(ticket: &Ticket, rules: &[Rule]) -> bool {
  !ticket
    .iter()
    .any(|x| !rules.iter().any(|r| rule_is_valid_for_field(&r.1, *x)))
}

fn rule_is_valid_for_field(rule: &[Range], field: u64) -> bool {
  rule.iter().any(|r| field >= r.min && field <= r.max)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> u64 {
  let rules = &input.rules;
  let valid_tickets = input
    .nearby_tickets
    .iter()
    .filter(|ticket| ticket_is_valid(ticket, rules))
    .collect::<Vec<_>>();

  let mut match_indices = rules
    .iter()
    .map(|r| collect_rule_match_indices(&valid_tickets, r))
    .collect::<Vec<_>>();

  let mut field_ordering: Vec<Option<String>> = vec![None; rules.len()];

  while match_indices.iter().any(|indices| indices.len() != 0) {
    let (rule_index, matches) = match_indices
      .iter()
      .enumerate()
      .find(|(_, matches)| matches.len() == 1)
      .unwrap();

    let match_index = *matches.iter().next().unwrap();

    for matches in &mut match_indices {
      matches.remove(&match_index);
    }

    field_ordering[match_index] = Some(rules[rule_index].0.clone());
  }

  let answer = input
    .my_ticket
    .iter()
    .zip(field_ordering.iter())
    .filter(|(_, field_name)| {
      field_name
        .as_ref()
        .map(|n| n.starts_with("departure"))
        .unwrap_or(false)
    })
    .map(|(field, _)| field)
    .product();

  return answer;
}

fn collect_rule_match_indices(tickets: &[&Ticket], rule: &Rule) -> HashSet<usize> {
  (0..tickets[0].len())
    .filter(|i| {
      tickets
        .iter()
        .all(|t| rule_is_valid_for_field(&rule.1, t[*i]))
    })
    .collect()
}

#[cfg(test)]
mod test {
  use super::{input_generator, solve_part1};
  const TEST_INPUT: &'static str = "class: 1-3 or 5-7\n\
  row: 6-11 or 33-44\n\
  seat: 13-40 or 45-50\n\
  \n\
  your ticket:\n\
  7,1,14\n\
  \n\
  nearby tickets:\n\
  7,3,47\n\
  40,4,50\n\
  55,2,20\n\
  38,6,12";

  #[test]
  fn test_day16_part1() {
    let input = &input_generator(TEST_INPUT);
    assert_eq!(solve_part1(&input), 71);
  }
}
