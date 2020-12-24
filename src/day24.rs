use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
	East,
	SouthEast,
	SouthWest,
	West,
	NorthWest,
	NorthEast,
}

type Tile = Vec<Direction>;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Tile> {
	let re = Regex::new(r"e|se|sw|w|nw|ne").unwrap();
	input
		.split("\n")
		.map(|line| {
			re.find_iter(line)
				.map(|dir| match dir.as_str() {
					"e" => Direction::East,
					"se" => Direction::SouthEast,
					"sw" => Direction::SouthWest,
					"w" => Direction::West,
					"nw" => Direction::NorthWest,
					"ne" => Direction::NorthEast,
					_ => unreachable!(),
				})
				.collect()
		})
		.collect()
}

type Pos = (i32, i32);

#[aoc(day24, part1)]
pub fn solve_part1(tiles: &[Tile]) -> usize {
	get_black_tile_set(tiles).len()
}

fn get_black_tile_set(tiles: &[Tile]) -> HashSet<Pos> {
	let mut black_tiles: HashSet<Pos> = HashSet::new();
	for tile in tiles.iter() {
		let mut q = 0;
		let mut r = 0;
		for direction in tile.iter() {
			match direction {
				Direction::East => {
					q += 1;
				}
				Direction::SouthEast => {
					r += 1;
				}
				Direction::SouthWest => {
					r += 1;
					q -= 1;
				}
				Direction::West => {
					q -= 1;
				}
				Direction::NorthWest => {
					r -= 1;
				}
				Direction::NorthEast => {
					r -= 1;
					q += 1;
				}
			}
		}
		let pos = (r, q);
		if black_tiles.contains(&pos) {
			black_tiles.remove(&pos);
		} else {
			black_tiles.insert(pos);
		}
	}

	return black_tiles;
}

#[aoc(day24, part2)]
pub fn solve_part2(tiles: &[Tile]) -> usize {
	let mut black_tiles = get_black_tile_set(tiles);
	hexa_conway_game_of_life(&mut black_tiles, 100)
}

fn hexa_conway_game_of_life(black_tiles: &mut HashSet<Pos>, cycles: usize) -> usize {
	const NEIGHBOUR_DIRECTIONS: [(i32, i32); 6] =
		[(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];
	let mut next_black_tiles: HashSet<Pos> = HashSet::new();

	let mut neighbour_count: HashMap<Pos, usize> = HashMap::new();

	for _ in 0..cycles {
		for (tx, ty) in black_tiles.iter() {
			for (dx, dy) in &NEIGHBOUR_DIRECTIONS {
				let n = (tx + dx, ty + dy);
				neighbour_count
					.entry(n)
					.and_modify(|c| *c += 1)
					.or_insert(1);
			}
		}

		next_black_tiles.clear();

		neighbour_count.drain().for_each(|(pos, c)| {
			if black_tiles.contains(&pos) {
				if c <= 2 {
					next_black_tiles.insert(pos);
				}
			} else {
				if c == 2 {
					next_black_tiles.insert(pos);
				}
			}
		});

		std::mem::swap(black_tiles, &mut next_black_tiles);
	}

	black_tiles.len()
}

#[cfg(test)]
mod test {
	use super::{input_generator, solve_part1, solve_part2};

	const TEST_INPUT: &'static str = "sesenwnenenewseeswwswswwnenewsewsw\n\
	neeenesenwnwwswnenewnwwsewnenwseswesw\n\
	seswneswswsenwwnwse\n\
	nwnwneseeswswnenewneswwnewseswneseene\n\
	swweswneswnenwsewnwneneseenw\n\
	eesenwseswswnenwswnwnwsewwnwsene\n\
	sewnenenenesenwsewnenwwwse\n\
	wenwwweseeeweswwwnwwe\n\
	wsweesenenewnwwnwsenewsenwwsesesenwne\n\
	neeswseenwwswnwswswnw\n\
	nenwswwsewswnenenewsenwsenwnesesenew\n\
	enewnwewneswsewnwswenweswnenwsenwsw\n\
	sweneswneswneneenwnewenewwneswswnese\n\
	swwesenesewenwneswnwwneseswwne\n\
	enesenwswwswneneswsenwnewswseenwsese\n\
	wnwnesenesenenwwnenwsewesewsesesew\n\
	nenewswnwewswnenesenwnesewesw\n\
	eneswnwswnwsenenwnwnwwseeswneewsenese\n\
	neswnwewnwnwseenwseesewsenwsweewe\n\
	wseweeenwnesenwwwswnew";

	#[test]
	fn test_day24_part1() {
		assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 10);
	}

	#[test]
	fn test_day24_part2() {
		assert_eq!(solve_part2(&input_generator(TEST_INPUT)), 2208);
	}
}
