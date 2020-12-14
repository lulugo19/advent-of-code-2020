use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
	SetMask(Mask),
	SetMemory { address: u64, value: u64 },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Mask {
	one_mask: u64,
	zero_mask: u64,
	floating_mask: u64,
}

impl FromStr for Mask {
	type Err = ();

	fn from_str(s: &str) -> Result<Mask, ()> {
		let one_mask = s
			.chars()
			.rev()
			.enumerate()
			.map(|(idx, c)| if c == '1' { 1 << (idx as u64) } else { 0 })
			.sum();

		let zero_mask = s
			.chars()
			.rev()
			.enumerate()
			.map(|(idx, c)| if c == '0' { 1 << (idx as u64) } else { 0 })
			.sum();

		let floating_mask = s
			.chars()
			.rev()
			.enumerate()
			.map(|(idx, c)| if c == 'X' { 1 << (idx as u64) } else { 0 })
			.sum();

		Ok(Mask {
			one_mask,
			zero_mask,
			floating_mask,
		})
	}
}

impl Mask {
	pub fn apply_part1(&self, value: u64) -> u64 {
		(value | self.one_mask) & (!self.zero_mask)
	}
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
	let re_mem = Regex::new(r"mem\[(\d+)\]").unwrap();

	input
		.lines()
		.map(|line| {
			let left_right: Vec<&str> = line.split(" = ").collect();
			let left = left_right[0];
			let right = left_right[1];
			if left == "mask" {
				Instruction::SetMask(Mask::from_str(right).unwrap())
			} else {
				let captures = re_mem.captures(left).unwrap();
				Instruction::SetMemory {
					address: captures[1].parse::<u64>().unwrap(),
					value: right.parse::<u64>().unwrap(),
				}
			}
		})
		.collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> u64 {
	let mut mem = HashMap::<u64, u64>::new();
	let mut mask: Option<Mask> = None;

	for instr in instructions {
		match instr {
			Instruction::SetMask(m) => mask = Some(m.clone()),
			Instruction::SetMemory { address, value } => {
				mem.insert(*address, mask.unwrap().apply_part1(*value));
			}
		}
	}

	mem.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> u64 {
	let mut mem = HashMap::<u64, u64>::new();
	let mut mask: Option<Mask> = None;

	for instr in instructions {
		match instr {
			Instruction::SetMask(m) => mask = Some(m.clone()),
			Instruction::SetMemory { address, value } => {
				let mask = mask.unwrap();
				let decoded_addr = address | mask.one_mask;
				let float_indices = get_set_bit_positions(mask.floating_mask);

				// use binary counting to get all the possible combinations of turning bits on or off
				// at the of float_indices
				let len = float_indices.len() as u64;
				let max = 1 << len;
				for i in 0..max {
					let mut decoded_addr = decoded_addr;
					for j in 0..len {
						if ((1 << j) & i) != 0 {
							decoded_addr |= (1 << float_indices[j as usize])
						} else {
							decoded_addr &= !(1 << float_indices[j as usize])
						};
					}
					mem.insert(decoded_addr, *value);
				}
			}
		}
	}

	mem.values().sum()
}

const NUM_BITS: u64 = 36;

fn get_set_bit_positions(mask: u64) -> Vec<u64> {
	let mut positions = Vec::<u64>::with_capacity(NUM_BITS as usize);
	for i in 0..NUM_BITS {
		if mask & (1 << i) != 0 {
			positions.push(i);
		}
	}
	positions
}

#[cfg(test)]
mod tests {
	use super::{input_generator, solve_part1, solve_part2, FromStr, Instruction, Mask};

	const TEST_INPUT_PART1: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
	mem[8] = 11\n\
	mem[7] = 101\n\
	mem[8] = 0";

	#[test]
	fn test_day14_input_generator() {
		let expected = vec![
			Instruction::SetMask(Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap()),
			Instruction::SetMemory {
				address: 8,
				value: 11,
			},
			Instruction::SetMemory {
				address: 7,
				value: 101,
			},
			Instruction::SetMemory {
				address: 8,
				value: 0,
			},
		];

		assert_eq!(input_generator(TEST_INPUT_PART1), expected);
	}

	#[test]
	fn test_day14_part1() {
		assert_eq!(solve_part1(&input_generator(TEST_INPUT_PART1)), 165)
	}

	#[test]
	fn test_day14_part2() {
		let input = "mask = 000000000000000000000000000000X1001X\n\
		mem[42] = 100\n\
		mask = 00000000000000000000000000000000X0XX\n\
		mem[26] = 1";

		assert_eq!(solve_part2(&input_generator(input)), 208)
	}
}
