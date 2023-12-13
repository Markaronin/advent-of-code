use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Stacks {
    val: BTreeMap<usize, Vec<char>>,
}
impl Stacks {
    pub fn new(lines: Vec<String>) -> Self {
        let len = (lines[0].len() / 4) + 1;
        let mut val = BTreeMap::new();
        for i in 1..=len {
            val.insert(i, vec![]);
        }

        let mut lines = lines;
        lines.pop();

        for line in lines {
            for (i, potential_crate) in line
                .chars()
                .chunks(4)
                .into_iter()
                .map(|c| c.collect::<String>())
                .enumerate()
            {
                match potential_crate.chars().nth(1).unwrap() {
                    ' ' => {}
                    c => val.get_mut(&(i + 1)).unwrap().insert(0, c),
                }
            }
        }

        Stacks { val }
    }

    pub fn process_instruction_9000(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.amount {
            let thing = self.val.get_mut(&instruction.from).unwrap().pop().unwrap();
            self.val.get_mut(&instruction.to).unwrap().push(thing);
        }
    }

    pub fn process_instruction_9001(&mut self, instruction: &Instruction) {
        let from_stack = self.val.get_mut(&instruction.from).unwrap();
        let things = from_stack.split_off(from_stack.len() - instruction.amount);
        self.val
            .get_mut(&instruction.to)
            .unwrap()
            .extend(things.iter());
    }

    pub fn top_crates(&self) -> Vec<char> {
        self.val
            .values()
            .map(|stack| *stack.last().unwrap())
            .collect_vec()
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}
impl Instruction {
    pub fn new(line: String) -> Self {
        let split = line.split_ascii_whitespace().collect_vec();
        Instruction {
            amount: split[1].parse().unwrap(),
            from: split[3].parse().unwrap(),
            to: split[5].parse().unwrap(),
        }
    }
}

fn get_program_output(input_file: &str) -> (String, String) {
    let input = read_lines(input_file);
    let mut stacks_input = vec![];
    let mut instructions_input = vec![];

    let mut half_found = false;
    for line in input {
        if !half_found {
            if line.is_empty() {
                half_found = true;
            } else {
                stacks_input.push(line);
            }
        } else {
            instructions_input.push(line);
        }
    }

    let stacks = Stacks::new(stacks_input);
    let instructions: Vec<Instruction> = instructions_input
        .into_iter()
        .map(Instruction::new)
        .collect();

    let mut stacks_1 = stacks.clone();
    for instruction in instructions.iter() {
        stacks_1.process_instruction_9000(instruction);
    }
    let result_1: String = stacks_1.top_crates().into_iter().collect();

    let mut stacks_2 = stacks.clone();
    for instruction in instructions.iter() {
        stacks_2.process_instruction_9001(instruction);
    }
    let result_2: String = stacks_2.top_crates().into_iter().collect();

    (result_1, result_2)
}

base_aoc!("CMZ", "MCD");
