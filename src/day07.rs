use std::collections::HashMap;
use std::collections::HashSet;

type Bags = HashMap<String, Vec<(usize, String)>>;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(usize, String)>> {
  input
    .lines()
    .map(|line| {
      let left_right = line.split(" contain ").collect::<Vec<_>>();
      let bag_color = left_right[0]
        .split(" ")
        .take(2)
        .collect::<Vec<_>>()
        .join(" ");

      let contains: Vec<(usize, String)> = if left_right[1].starts_with("no") {
        Vec::new()
      } else {
        left_right[1]
          .split(", ")
          .map(|s| {
            let tokens = s.split(" ").collect::<Vec<_>>();
            (
              tokens[0].parse::<usize>().unwrap(),
              tokens[1].to_string() + " " + tokens[2],
            )
          })
          .collect()
      };

      (bag_color, contains)
    })
    .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(bags: &Bags) -> usize {
  let mut count = 0;
  for bag in bags.keys() {
    let mut visited = HashSet::new();
    if contains_bag(bags, bag, "shiny gold", &mut visited) {
      count += 1;
    }
  }

  count
}

fn contains_bag(bags: &Bags, bag: &str, contains: &str, visited: &mut HashSet<String>) -> bool {
  let bags_contained = bags.get(bag).unwrap();

  if bags_contained.len() == 0 {
    false
  } else {
    bags_contained.iter().any(|(_, bag)| {
      if bag == contains {
        true
      } else if visited.contains(bag) {
        false
      } else {
        visited.insert(bag.to_string());
        contains_bag(bags, bag, contains, visited)
      }
    })
  }
}

#[aoc(day7, part2)]
pub fn solve_part2(bags: &Bags) -> usize {
  count_bags(bags, "shiny gold") - 1
}

fn count_bags(bags: &Bags, bag: &str) -> usize {
  let bags_contained = bags.get(bag).unwrap();

  1 + bags_contained
    .iter()
    .map(|(count, bag)| count * count_bags(bags, bag))
    .sum::<usize>()
}
