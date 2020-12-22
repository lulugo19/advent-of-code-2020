use std::collections::{HashSet, VecDeque};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (VecDeque<u16>, VecDeque<u16>) {
	let mut players = input.split("\n\n");

	let p1 = players
		.next()
		.unwrap()
		.split("\n")
		.skip(1)
		.map(|line| line.parse().unwrap())
		.collect();

	let p2 = players
		.next()
		.unwrap()
		.split("\n")
		.skip(1)
		.map(|line| line.parse().unwrap())
		.collect();

	return (p1, p2);
}

#[aoc(day22, part1)]
pub fn solve_part1((p1, p2): &(VecDeque<u16>, VecDeque<u16>)) -> u16 {
	let mut p1 = p1.clone();
	let mut p2 = p2.clone();

	while p1.len() > 0 && p2.len() > 0 {
		let p1_card = p1.pop_front().unwrap();
		let p2_card = p2.pop_front().unwrap();

		if p1_card > p2_card {
			p1.push_back(p1_card);
			p1.push_back(p2_card);
		} else {
			p2.push_back(p2_card);
			p2.push_back(p1_card);
		}
	}

	let winner = if p1.len() > 0 { p1 } else { p2 };

	return score(&winner);
}

fn score(cards: &VecDeque<u16>) -> u16 {
	cards
		.iter()
		.rev()
		.enumerate()
		.map(|(i, v)| *v * (i + 1) as u16)
		.sum()
}

#[aoc(day22, part2)]
pub fn solve_part2((p1, p2): &(VecDeque<u16>, VecDeque<u16>)) -> u16 {
	let (_, winner_deck) = recursive_combat((p1.clone(), p2.clone()));
	score(&winner_deck)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Winner {
	Player1,
	Player2,
}

pub fn recursive_combat(
	(mut p1, mut p2): (VecDeque<u16>, VecDeque<u16>),
) -> (Winner, VecDeque<u16>) {
	let mut prev_p1_rounds: HashSet<VecDeque<u16>> = HashSet::new();
	let mut prev_p2_rounds: HashSet<VecDeque<u16>> = HashSet::new();

	while p1.len() > 0 && p2.len() > 0 {
		if prev_p1_rounds.contains(&p1) || prev_p2_rounds.contains(&p2) {
			return (Winner::Player1, p1);
		}

		prev_p1_rounds.insert(p1.clone());
		prev_p2_rounds.insert(p2.clone());

		let p1_card = p1.pop_front().unwrap();
		let p2_card = p2.pop_front().unwrap();

		let winner = if p1.len() >= p1_card as usize && p2.len() >= p2_card as usize {
			let (winner, _) = recursive_combat((
				p1.iter().take(p1_card as usize).cloned().collect(),
				p2.iter().take(p2_card as usize).cloned().collect(),
			));
			winner
		} else if p1_card > p2_card {
			Winner::Player1
		} else {
			Winner::Player2
		};

		if winner == Winner::Player1 {
			p1.push_back(p1_card);
			p1.push_back(p2_card);
		} else {
			p2.push_back(p2_card);
			p2.push_back(p1_card);
		}
	}

	if p1.len() > 0 {
		(Winner::Player1, p1)
	} else {
		(Winner::Player2, p2)
	}
}

#[cfg(test)]
mod test {

	use super::{input_generator, solve_part1, solve_part2};

	const TEST_INPUT: &'static str = "Player 1:\n\
	9\n\
	2\n\
	6\n\
	3\n\
	1\n\
	\n\
	Player 2:\n\
	5\n\
	8\n\
	4\n\
	7\n\
	10";

	#[test]
	fn test_day22_part1() {
		assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 306);
	}

	#[test]
	fn test_day22_part2() {
		assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 291);
	}
}
