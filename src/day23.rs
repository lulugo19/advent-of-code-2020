use std::iter;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<usize> {
	input
		.chars()
		.map(|x| x.to_string().parse().unwrap())
		.collect()
}

#[aoc(day23, part1)]
fn solve_part1(cups: &[usize]) -> String {
	let succ = cup_game_fast(cups, 100);

	let mut cup_str = String::new();

	let mut i = 1;
	for _ in 0..cups.len() - 1 {
		i = succ[i];
		cup_str.push_str(&i.to_string());
	}

	return cup_str;
}

#[aoc(day23, part2)]
fn solve_part2(_cups: &[usize]) -> usize {
	let mut cups: Vec<usize> = Vec::with_capacity(1_000_000);
	cups.extend(_cups.iter());
	let max = cups.iter().max().unwrap();
	for i in max + 1..=1_000_000 {
		cups.push(i);
	}

	let succ = cup_game_fast(&cups, 10_000_000);

	let c1 = succ[1];
	let c2 = succ[c1];

	return c1 * c2;
}

// slow version of the cup game, with cups just in vec and vec manipulations
#[allow(dead_code)]
fn cup_game_slow(cups: &[usize], num_moves: usize) -> String {
	let mut cups: Vec<usize> = cups.iter().cloned().collect();
	let l = cups.len();

	for _ in 0..num_moves {
		println!("cups: {:?}", cups);
		let mut rem_cups: Vec<usize> = cups.splice(0..4, iter::empty()).collect();

		let curr = rem_cups.remove(0);

		let mut dest = if curr == 1 { l } else { curr - 1 };
		while rem_cups.contains(&dest) {
			if dest == 1 {
				dest = l;
			} else {
				dest -= 1;
			}
		}

		let dest_index = cups.iter().position(|x| *x == dest).unwrap() + 1;
		cups.splice(dest_index..dest_index, rem_cups.into_iter());
		cups.push(curr);
	}

	let one_index = cups.iter().position(|x| *x == 1).unwrap();

	cups
		.into_iter()
		.cycle()
		.skip(one_index + 1)
		.take(l - 1)
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
		.join("")
}

// fast version of the cup game, storing the successor of a cup in a vec
fn cup_game_fast(cups: &[usize], num_moves: usize) -> Vec<usize> {
	let mut curr = cups[0].clone();

	// add plus one to the cup size sow that we have a quasi one index array
	let mut succ: Vec<usize> = vec![0; cups.len() + 1];

	for i in 0..cups.len() {
		succ[cups[i]] = cups[(i + 1) % cups.len()];
	}

	for _ in 0..num_moves {
		let c1 = succ[curr];
		let c2 = succ[c1];
		let c3 = succ[c2];

		let mut dest = if curr == 1 { cups.len() } else { curr - 1 };
		while dest == c1 || dest == c2 || dest == c3 {
			dest = if dest == 1 { cups.len() } else { dest - 1 };
		}
		succ[curr] = succ[c3];
		succ[c3] = succ[dest];
		succ[dest] = c1;

		curr = succ[curr];
	}

	return succ;
}
