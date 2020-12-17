use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec3 {
	x: i32,
	y: i32,
	z: i32,
}

impl Vec3 {
	pub fn new(x: i32, y: i32, z: i32) -> Vec3 {
		Vec3 { x, y, z }
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec4 {
	x: i32,
	y: i32,
	z: i32,
	w: i32,
}

impl Vec4 {
	pub fn new(x: i32, y: i32, z: i32, w: i32) -> Vec4 {
		Vec4 { x, y, z, w }
	}
}

#[aoc_generator(day17, part1)]
pub fn input_generator_part1(input: &str) -> HashSet<Vec3> {
	let mut active = HashSet::<Vec3>::new();
	input.lines().enumerate().for_each(|(y, line)| {
		line.chars().enumerate().for_each(|(x, c)| {
			if c == '#' {
				active.insert(Vec3 {
					x: x as i32,
					y: y as i32,
					z: 0,
				});
			}
		})
	});
	return active;
}

#[aoc_generator(day17, part2)]
pub fn input_generator_part2(input: &str) -> HashSet<Vec4> {
	let mut active = HashSet::<Vec4>::new();
	input.lines().enumerate().for_each(|(y, line)| {
		line.chars().enumerate().for_each(|(x, c)| {
			if c == '#' {
				active.insert(Vec4 {
					x: x as i32,
					y: y as i32,
					z: 0,
					w: 0,
				});
			}
		})
	});
	return active;
}

#[aoc(day17, part1)]
pub fn solve_part1(initial_active: &HashSet<Vec3>) -> usize {
	conway_3d(initial_active, 6)
}

fn conway_3d(initial_active: &HashSet<Vec3>, cycles: usize) -> usize {
	let mut active = initial_active.clone();
	let mut next_active = HashSet::<Vec3>::new();

	for i in 0..cycles {
		println!("cycle: {} {}", i, active.len());
		next_active.clear();
		let min_x = active.iter().map(|v| v.x).min().unwrap() - 1;
		let min_y = active.iter().map(|v| v.y).min().unwrap() - 1;
		let min_z = active.iter().map(|v| v.z).min().unwrap() - 1;

		let max_x = active.iter().map(|v| v.x).max().unwrap() + 1;
		let max_y = active.iter().map(|v| v.y).max().unwrap() + 1;
		let max_z = active.iter().map(|v| v.z).max().unwrap() + 1;

		for x in min_x..=max_x {
			for y in min_y..=max_y {
				for z in min_z..=max_z {
					let pos = Vec3::new(x, y, z);
					let is_active = active.contains(&pos);

					let mut n = 0;
					for nx in x - 1..=x + 1 {
						for ny in y - 1..=y + 1 {
							for nz in z - 1..=z + 1 {
								let n_pos = Vec3::new(nx, ny, nz);
								if pos == n_pos {
									continue;
								}
								if active.contains(&n_pos) {
									n += 1;
								}
							}
						}
					}

					if is_active && (n == 2 || n == 3) {
						next_active.insert(pos);
					} else if !is_active && n == 3 {
						next_active.insert(pos);
					}
				}
			}
		}
		std::mem::swap(&mut active, &mut next_active);
	}

	return active.len();
}

#[aoc(day17, part2)]
pub fn solve_part2(initial_active: &HashSet<Vec4>) -> usize {
	conway_4d(initial_active, 6)
}

fn conway_4d(initial_active: &HashSet<Vec4>, cycles: usize) -> usize {
	let mut active = initial_active.clone();
	let mut next_active = HashSet::<Vec4>::new();

	for i in 0..cycles {
		println!("cycle: {} {}", i, active.len());
		next_active.clear();
		let min_x = active.iter().map(|v| v.x).min().unwrap() - 1;
		let min_y = active.iter().map(|v| v.y).min().unwrap() - 1;
		let min_z = active.iter().map(|v| v.z).min().unwrap() - 1;
		let min_w = active.iter().map(|v| v.w).min().unwrap() - 1;

		let max_x = active.iter().map(|v| v.x).max().unwrap() + 1;
		let max_y = active.iter().map(|v| v.y).max().unwrap() + 1;
		let max_z = active.iter().map(|v| v.z).max().unwrap() + 1;
		let max_w = active.iter().map(|v| v.w).max().unwrap() + 1;

		for x in min_x..=max_x {
			for y in min_y..=max_y {
				for z in min_z..=max_z {
					for w in min_w..=max_w {
						let pos = Vec4::new(x, y, z, w);
						let is_active = active.contains(&pos);

						let mut n = 0;
						for nx in x - 1..=x + 1 {
							for ny in y - 1..=y + 1 {
								for nz in z - 1..=z + 1 {
									for nw in w - 1..=w + 1 {
										let n_pos = Vec4::new(nx, ny, nz, nw);
										if pos == n_pos {
											continue;
										}
										if active.contains(&n_pos) {
											n += 1;
										}
									}
								}
							}
						}

						if is_active && (n == 2 || n == 3) {
							next_active.insert(pos);
						} else if !is_active && n == 3 {
							next_active.insert(pos);
						}
					}
				}
			}
		}
		std::mem::swap(&mut active, &mut next_active);
	}

	return active.len();
}
