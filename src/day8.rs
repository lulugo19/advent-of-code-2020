use crate::NoSolutionErr;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            "nop" => Ok(Operation::Nop),
            _ => Err(()),
        }
    }
}

type Instruction = (Operation, i32);

type Program = Vec<Instruction>;

#[aoc_generator(day8)]
pub fn generate_input(input: &str) -> Program {
    input
        .lines()
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            (
                Operation::from_str(tokens[0]).unwrap(),
                tokens[1].parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(program: &Program) -> i32 {
    match execute_program(program) {
        Err(acc) => acc,
        _ => panic!("The progam should be stuck in an infinite loop!"),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(program: &Program) -> Result<i32, NoSolutionErr> {
    let mut program = program.clone();
    let mut change_index = 0;
    while change_index < program.len() {
        while change_index < program.len() && program[change_index].0 == Operation::Acc {
            change_index += 1;
        }
        if change_index >= program.len() {
            break;
        }
        let prev_instr = program[change_index];
        let changed_op = match prev_instr.0 {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            _ => panic!("Should be either Jmp or Nop"),
        };
        let new_instr = (changed_op, prev_instr.1);
        program[change_index] = new_instr;

        match execute_program(&program) {
            Ok(acc) => {
                println!(
                    "Found corrupted instruction on line {}: {:?}\n\
                     Set to new instruction: {:?}\n\
                     acc is '{}'",
                    change_index, prev_instr, new_instr, acc
                );
                return Ok(acc);
            }
            Err(_) => {
                // reset instruction
                program[change_index] = prev_instr;
                change_index += 1;
            }
        }
    }
    Err(NoSolutionErr {})
}

fn execute_program(program: &Program) -> Result<i32, i32> {
    let mut instr_ptr: usize = 0;
    let mut executed_instructions = HashSet::<usize>::new();
    let mut acc = 0;

    while !executed_instructions.contains(&instr_ptr) && instr_ptr < program.len() {
        executed_instructions.insert(instr_ptr);
        match program[instr_ptr] {
            (Operation::Acc, arg) => {
                acc += arg;
            }
            (Operation::Jmp, arg) => {
                let new_instr_ptr = instr_ptr as i32 + arg;
                if new_instr_ptr < 0 {
                    panic!("Negative Instruction Ptr");
                }
                instr_ptr = new_instr_ptr as usize;
                continue;
            }
            (Operation::Nop, _) => {}
        }
        instr_ptr += 1;
    }
    if instr_ptr == program.len() {
        Ok(acc)
    } else {
        Err(acc)
    }
}
