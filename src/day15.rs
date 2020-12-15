use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
	input
		.split(",")
		.map(|s| s.parse::<usize>().unwrap())
		.collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(start_numbers: &[usize]) -> usize {
	play_game(start_numbers, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(start_numbers: &[usize]) -> usize {
	play_game(start_numbers, 30_000_000)
}

fn play_game(start_numbers: &[usize], nth: usize) -> usize {
	if nth < start_numbers.len() {
		return start_numbers[nth - 1];
	}

	let mut last_spoken: HashMap<usize, usize> = HashMap::new();

	for (idx, n) in start_numbers
		.iter()
		.take(start_numbers.len() - 1)
		.enumerate()
	{
		last_spoken.insert(*n, idx);
	}

	let mut last = *start_numbers.last().unwrap();

	for i in start_numbers.len()..nth {
		let spoken_before = if let Some(j) = last_spoken.get(&last) {
			i - j - 1
		} else {
			0
		};

		last_spoken.insert(last, i - 1);
		last = spoken_before;
	}

	last
}

#[cfg(test)]
pub mod test {
	use super::{input_generator, solve_part1};

	#[test]
	fn test_day15_part1() {
		assert_eq!(solve_part1(&input_generator("0,3,6")), 436);
	}
}
