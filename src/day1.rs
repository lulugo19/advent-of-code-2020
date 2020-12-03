use std::collections::HashSet;

const GOAL: i64 = 2020;

#[aoc_generator(day1)]
pub fn input_generator_day1(input: &str) -> HashSet<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &HashSet<i64>) -> Option<i64> {
    sum2(&input, GOAL).map(|(a, b)| a * b)
}

fn sum2(set: &HashSet<i64>, goal: i64) -> Option<(i64, i64)> {
    for &a in set.iter() {
        let b = goal - a;
        if a != b && set.contains(&b) {
            return Some((a, b));
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &HashSet<i64>) -> Option<i64> {
    sum3(input, GOAL).map(|(a, b, c)| a * b * c)
}

fn sum3(set: &HashSet<i64>, goal: i64) -> Option<(i64, i64, i64)> {
    for &a in set.iter() {
        let to_goal = goal - a;
        if let Some((b, c)) = sum2(set, to_goal) {
            if a != b && a != c {
                return Some((a, b, c));
            }
        }
    }
    None
}
