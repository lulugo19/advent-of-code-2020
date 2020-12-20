use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tile {
	id: u64,
	data: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Border {
	Top,
	Right,
	Bottom,
	Left,
}

impl Border {
	pub fn opposite(self) -> Border {
		match self {
			Border::Top => Border::Bottom,
			Border::Right => Border::Left,
			Border::Bottom => Border::Top,
			Border::Left => Border::Right,
		}
	}
}

const BORDERS: [Border; 4] = [Border::Top, Border::Right, Border::Bottom, Border::Left];

impl Tile {
	pub fn width(&self) -> usize {
		self.data[0].len()
	}

	pub fn height(&self) -> usize {
		self.data.len()
	}

	pub fn flip(&mut self) {
		self.data.iter_mut().for_each(|row| row.reverse())
	}

	pub fn rotate_cw(&mut self) {
		self.data = (0..self.width())
			.map(|col| {
				(0..self.height())
					.rev()
					.map(|row| self.data[row][col])
					.collect()
			})
			.collect();
	}

	pub fn get_border(&self, border: Border) -> String {
		match border {
			Border::Top => self.data[0].iter().collect(),
			Border::Bottom => self.data[self.height() - 1].iter().collect(),
			Border::Left => self.data.iter().map(|row| row[0]).collect(),
			Border::Right => self.data.iter().map(|row| row[self.width() - 1]).collect(),
		}
	}

	pub fn matches_tile(&self, tile: &Tile) -> Option<Border> {
		for &border in &BORDERS {
			if self.get_border(border) == tile.get_border(border.opposite()) {
				return Some(border);
			}
		}
		return None;
	}
}

impl FromStr for Tile {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let re = Regex::new(r"^Tile (\d+):$").unwrap();
		let mut lines = s.split("\n");
		let id = re
			.captures(lines.next().unwrap())
			.unwrap()
			.get(1)
			.unwrap()
			.as_str()
			.parse::<u64>()
			.unwrap();

		let data = lines.map(|line| line.chars().collect()).collect();

		Ok(Tile { id, data })
	}
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(
			&self
				.data
				.iter()
				.map(|row| row.iter().collect::<String>())
				.collect::<Vec<_>>()
				.join("\n"),
		)
	}
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Tile> {
	input
		.split("\n\n")
		.map(|tile| Tile::from_str(tile).unwrap())
		.collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(tiles: &[Tile]) -> u64 {
	let puzzle = solve_puzzle(tiles);
	let l = puzzle.len() - 1;

	[(0, 0), (0, l), (l, 0), (l, l)]
		.iter()
		.map(|&(row, col)| puzzle[row][col].as_ref().unwrap().id)
		.product()
}

#[aoc(day20, part2)]
pub fn solve_part2(tiles: &[Tile]) -> usize {
	let puzzle = solve_puzzle(tiles);
	let puzzle_size = puzzle.len();
	let tile_size = puzzle[0][0].as_ref().unwrap().data.len() - 2;
	let row_count = puzzle_size * tile_size;
	let mut sea_picture = Vec::<String>::with_capacity(row_count + 1);
	sea_picture.push(String::from("Tile 1000:"));

	for row in 0..row_count {
		let mut row_str = String::new();
		for col in 0..puzzle_size {
			let tile = puzzle[row / tile_size][col].as_ref().unwrap();
			row_str.push_str(
				&tile.data[1 + row % tile_size]
					.iter()
					.skip(1)
					.take(tile_size)
					.collect::<String>(),
			);
		}
		sea_picture.push(row_str);
	}

	let sea_picture = sea_picture.join("\n");
	let total_hashtag_count = sea_picture.chars().filter(|c| *c == '#').count();
	let mut sea_picture = Tile::from_str(&sea_picture).unwrap();

	const SEA_MONSTER: &str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";

	let monster_hashtag_count = SEA_MONSTER.chars().filter(|c| *c == '#').count();
	let monster_width = SEA_MONSTER.split("\n").next().unwrap().len();
	let monster_height = SEA_MONSTER.split("\n").count();

	let mut monster_indices: Vec<(usize, usize)> = Vec::new();
	SEA_MONSTER.split("\n").enumerate().for_each(|(y, line)| {
		line.chars().enumerate().for_each(|(x, c)| {
			if c == '#' {
				monster_indices.push((x, y));
			}
		})
	});

	let mut monster_count = 0;

	for i in 0..8 {
		for row in 0..sea_picture.height() - monster_height {
			for col in 0..sea_picture.width() - monster_width {
				if monster_indices
					.iter()
					.all(|(x, y)| sea_picture.data[row + y][col + x] == '#')
				{
					monster_count += 1;
				}
			}
		}

		if monster_count > 0 {
			break;
		}

		sea_picture.rotate_cw();

		if i == 3 {
			sea_picture.flip();
		}
	}

	println!("{} monsters found!", monster_count);

	let roughness = total_hashtag_count - monster_hashtag_count * monster_count;
	return roughness;
}

fn solve_puzzle(tiles: &[Tile]) -> Vec<Vec<Option<Tile>>> {
	let side_length = f64::sqrt(tiles.len() as f64) as usize;
	println!("puzzle side length: {}", side_length);

	let mut remaining_tiles = tiles.iter().cloned().collect::<Vec<_>>();
	let mut puzzle: Vec<Vec<Option<Tile>>> = vec![vec![None; side_length]; side_length];

	puzzle[side_length / 2][side_length / 2] = Some(remaining_tiles.pop().unwrap());

	while remaining_tiles.len() > 0 {
		let mut next_tile = Some(remaining_tiles.pop().unwrap());

		for row in 0..side_length {
			for col in 0..side_length {
				let mut match_border: Option<Border> = None;
				if let Some(piece) = &mut puzzle[row][col] {
					for i in 0..8 {
						let border = piece.matches_tile(next_tile.as_ref().unwrap());
						if border.is_some() {
							match_border = border;
							break;
						}
						next_tile.as_mut().unwrap().rotate_cw();
						if i == 3 {
							next_tile.as_mut().unwrap().flip();
						}
					}
				}

				if let Some(border) = match_border {
					println!(
						"match piece {:?} at ({:?} {:?} {:?})",
						next_tile.as_ref().unwrap().id,
						row,
						col,
						border
					);
					match border {
						Border::Top => {
							if row == 0 {
								for row in (1..side_length).rev() {
									for col in 0..side_length {
										puzzle[row][col] = puzzle[row - 1][col].take();
									}
								}
								puzzle[row][col] = next_tile.take();
							} else {
								puzzle[row - 1][col] = next_tile.take();
							}
						}
						Border::Right => {
							if col == side_length - 1 {
								for row in 0..side_length {
									for col in 0..side_length - 1 {
										puzzle[row][col] = puzzle[row][col + 1].take();
									}
								}
								puzzle[row][col] = next_tile.take();
							} else {
								puzzle[row][col + 1] = next_tile.take();
							}
						}
						Border::Bottom => {
							if row == side_length - 1 {
								for row in 0..side_length - 1 {
									for col in 0..side_length {
										puzzle[row][col] = puzzle[row + 1][col].take();
									}
								}
								puzzle[row][col] = next_tile.take();
							} else {
								puzzle[row + 1][col] = next_tile.take();
							}
						}
						Border::Left => {
							if col == 0 {
								for row in 0..side_length {
									for col in (1..side_length).rev() {
										puzzle[row][col] = puzzle[row][col - 1].take();
									}
								}
								puzzle[row][col] = next_tile.take();
							} else {
								puzzle[row][col - 1] = next_tile.take();
							}
						}
					}
					break;
				}
			}
			if next_tile.is_none() {
				break;
			}
		}

		if next_tile.is_some() {
			// Piece couldn't be matched! so insert it back!
			remaining_tiles.insert(0, next_tile.unwrap());
		}
	}

	puzzle
}

#[cfg(test)]
mod test {

	use super::{input_generator, solve_part1, solve_part2, Tile};
	use std::str::FromStr;

	const TEST_INPUT: &'static str = "Tile 1234:\n\
	..#.\n\
	##.#\n\
	..##\n\
	####";

	#[test]
	fn test_day20_rotate_tile() {
		let mut tile = Tile::from_str(TEST_INPUT).unwrap();
		tile.rotate_cw();
		let expected = "#.#.\n#.#.\n##.#\n###.";
		assert_eq!(tile.to_string(), expected);
	}

	#[test]
	fn test_day20_flip_tile() {
		let mut tile = Tile::from_str(TEST_INPUT).unwrap();
		tile.flip();
		let expected = ".#..\n#.##\n##..\n####";
		assert_eq!(tile.to_string(), expected);
	}

	const TEST_PUZZLE: &'static str = "Tile 2311:\n\
	..##.#..#.\n\
	##..#.....\n\
	#...##..#.\n\
	####.#...#\n\
	##.##.###.\n\
	##...#.###\n\
	.#.#.#..##\n\
	..#....#..\n\
	###...#.#.\n\
	..###..###\n\
	\n\
	Tile 1951:\n\
	#.##...##.\n\
	#.####...#\n\
	.....#..##\n\
	#...######\n\
	.##.#....#\n\
	.###.#####\n\
	###.##.##.\n\
	.###....#.\n\
	..#.#..#.#\n\
	#...##.#..\n\
	\n\
	Tile 1171:\n\
	####...##.\n\
	#..##.#..#\n\
	##.#..#.#.\n\
	.###.####.\n\
	..###.####\n\
	.##....##.\n\
	.#...####.\n\
	#.##.####.\n\
	####..#...\n\
	.....##...\n\
	\n\
	Tile 1427:\n\
	###.##.#..\n\
	.#..#.##..\n\
	.#.##.#..#\n\
	#.#.#.##.#\n\
	....#...##\n\
	...##..##.\n\
	...#.#####\n\
	.#.####.#.\n\
	..#..###.#\n\
	..##.#..#.\n\
	\n\
	Tile 1489:\n\
	##.#.#....\n\
	..##...#..\n\
	.##..##...\n\
	..#...#...\n\
	#####...#.\n\
	#..#.#.#.#\n\
	...#.#.#..\n\
	##.#...##.\n\
	..##.##.##\n\
	###.##.#..\n\
	\n\
	Tile 2473:\n\
	#....####.\n\
	#..#.##...\n\
	#.##..#...\n\
	######.#.#\n\
	.#...#.#.#\n\
	.#########\n\
	.###.#..#.\n\
	########.#\n\
	##...##.#.\n\
	..###.#.#.\n\
	\n\
	Tile 2971:\n\
	..#.#....#\n\
	#...###...\n\
	#.#.###...\n\
	##.##..#..\n\
	.#####..##\n\
	.#..####.#\n\
	#..#.#..#.\n\
	..####.###\n\
	..#.#.###.\n\
	...#.#.#.#\n\
	\n\
	Tile 2729:\n\
	...#.#.#.#\n\
	####.#....\n\
	..#.#.....\n\
	....#..#.#\n\
	.##..##.#.\n\
	.#.####...\n\
	####.#.#..\n\
	##.####...\n\
	##..#.##..\n\
	#.##...##.\n\
	\n\
	Tile 3079:\n\
	#.#.#####.\n\
	.#..######\n\
	..#.......\n\
	######....\n\
	####.#..#.\n\
	.#...#.##.\n\
	#.#####.##\n\
	..#.###...\n\
	..#.......\n\
	..#.###...";

	#[test]
	fn test_day20_part1() {
		let tiles = input_generator(TEST_PUZZLE);
		assert_eq!(solve_part1(&tiles), 20899048083289);
	}

	#[test]
	fn test_day20_part2() {
		let tiles = input_generator(TEST_PUZZLE);
		assert_eq!(solve_part2(&tiles), 273);
	}
}
