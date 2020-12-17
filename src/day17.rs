use std::collections::{HashMap, HashSet};

pub fn input_generator(input: &str, n: usize) -> HashSet<Vec<i32>> {
	let mut active = HashSet::<Vec<i32>>::new();
	input.lines().enumerate().for_each(|(y, line)| {
		line.chars().enumerate().for_each(|(x, c)| {
			if c == '#' {
				let mut v = vec![0; n];
				v[0] = x as i32;
				v[1] = y as i32;
				active.insert(v);
			}
		})
	});
	return active;
}

#[aoc_generator(day17, part1)]
pub fn input_generator_part1(input: &str) -> HashSet<Vec<i32>> {
	input_generator(input, 3)
}

#[aoc_generator(day17, part2)]
pub fn input_generator_part2(input: &str) -> HashSet<Vec<i32>> {
	input_generator(input, 4)
}

#[aoc(day17, part1)]
pub fn solve_part1(initial_active: &HashSet<Vec<i32>>) -> usize {
	*conway_nd(3, initial_active, 6).last().unwrap()
}

#[aoc(day17, part2)]
pub fn solve_part2(initial_active: &HashSet<Vec<i32>>) -> usize {
	*conway_nd(4, initial_active, 6).last().unwrap()
}

pub fn conway_nd(n: usize, initial_active: &HashSet<Vec<i32>>, cycles: usize) -> Vec<usize> {
	let mut active_count = vec![initial_active.len()];
	println!("intitial active: {}", initial_active.len());
	let mut active = initial_active.clone();
	let mut next_active = HashSet::<Vec<i32>>::new();

	let neighbour_count: u128 = u128::pow(3, n as u32);
	let neighbour_offsets = (0..neighbour_count)
		.map(|i| {
			let mut x = i;
			let mut offset = Vec::with_capacity(n);
			for _j in 0..n {
				offset.push((x % 3) as i32 - 1);
				x /= 3;
			}
			offset
		})
		.collect::<Vec<_>>();

	let mut active_neighbour_count: HashMap<Vec<i32>, u32> = HashMap::new();
	let mut inactive_neighbour_count: HashMap<Vec<i32>, u32> = HashMap::new();

	for i in 0..cycles {
		next_active.clear();

		for pos in active.iter() {
			for offset in neighbour_offsets.iter() {
				let neighbour = pos
					.iter()
					.zip(offset.iter())
					.map(|(p, o)| p + o)
					.collect::<Vec<_>>();
				if *pos == neighbour {
					continue;
				}
				if active.contains(&neighbour) {
					*active_neighbour_count.entry(neighbour).or_insert(0) += 1;
				} else {
					*inactive_neighbour_count.entry(neighbour).or_insert(0) += 1;
				}
			}
		}

		next_active.extend(active_neighbour_count.drain().filter_map(|(pos, count)| {
			if count == 2 || count == 3 {
				Some(pos)
			} else {
				None
			}
		}));

		next_active.extend(inactive_neighbour_count.drain().filter_map(|(pos, count)| {
			if count == 3 {
				Some(pos)
			} else {
				None
			}
		}));

		std::mem::swap(&mut active, &mut next_active);

		active_count.push(active.len());
		println!("cycle {}: {} active", i, active.len());
	}

	return active_count;
}

#[cfg(test)]
pub mod test {

	use super::{input_generator_part1, input_generator_part2, solve_part1, solve_part2};

	const TEST_INPUT: &'static str = ".#.\n\
	..#\n\
	###";

	#[test]
	fn test_day17_part1() {
		assert_eq!(solve_part1(&input_generator_part1(TEST_INPUT)), 112);
	}

	#[test]
	fn test_day17_part2() {
		assert_eq!(solve_part2(&input_generator_part2(TEST_INPUT)), 848);
	}
}
