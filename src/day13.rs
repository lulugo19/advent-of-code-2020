#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
	earliest_timestamp: i128,
	busses: Vec<Option<i128>>,
}

impl Input {
	pub fn new_part2(busses: Vec<Option<i128>>) -> Input {
		Input {
			earliest_timestamp: 0,
			busses,
		}
	}
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
	let lines = input.lines().collect::<Vec<&str>>();
	let earliest_timestamp = lines[0].parse::<i128>().unwrap();
	let busses = lines[1]
		.split(",")
		.map(|string| match string {
			"x" => None,
			_ => Some(string.parse::<i128>().unwrap()),
		})
		.collect();

	Input {
		earliest_timestamp,
		busses,
	}
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> i128 {
	let busses: Vec<i128> = input.busses.iter().filter_map(|x| x.clone()).collect();

	let mut t = input.earliest_timestamp;
	loop {
		if let Some(id) = busses.iter().find(|id| t % *id == 0) {
			let minutes_to_wait = t - input.earliest_timestamp;
			return id * minutes_to_wait;
		}
		t += 1;
	}
}

// solving part2 using the chinese remainder theorem
// https://en.wikipedia.org/wiki/Chinese_remainder_theorem
// (t + i) = 0 mod id <=>
// t = -i mod id
#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> i128 {
	let equations: Vec<(i128, i128)> = input
		.busses
		.iter()
		.enumerate()
		.filter_map(|(idx, bus)| bus.map(|b| ((b - idx as i128) % b, b)))
		.collect();

	let M: i128 = equations.iter().map(|(a, m)| *m).product();

	let mut result = equations
		.iter()
		.map(|(a, m)| {
			let mut x = 0;
			let mut y = 0;
			let N = M / m;
			gcd_extended(*m, N, &mut x, &mut y);
			a * N * y
		})
		.sum::<i128>()
		% M;

	if result < 0 {
		result += M;
	}

	result
}

fn gcd_extended(a: i128, b: i128, x: &mut i128, y: &mut i128) -> i128 {
	if a == 0 {
		*x = 0;
		*y = 1;
		return b;
	}
	let mut x1: i128 = 0;
	let mut y1: i128 = 0;

	let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

	*x = y1 - (b / a) * x1;
	*y = x1;

	return gcd;
}

#[cfg(test)]
pub mod tests {

	use super::{solve_part2, Input};

	#[test]
	fn test_day13_part2() {
		assert_eq!(
			solve_part2(&Input::new_part2(vec![Some(17), None, Some(13), Some(19)])),
			3417
		);

		assert_eq!(
			solve_part2(&Input::new_part2(vec![
				Some(67),
				Some(7),
				Some(59),
				Some(61)
			])),
			754018
		);

		assert_eq!(
			solve_part2(&Input::new_part2(vec![
				Some(67),
				Some(7),
				None,
				Some(59),
				Some(61)
			])),
			1261476
		);

		assert_eq!(
			solve_part2(&Input::new_part2(vec![
				Some(1789),
				Some(37),
				Some(47),
				Some(1889)
			])),
			1202161486
		);
	}
}
