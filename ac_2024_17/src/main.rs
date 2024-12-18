use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::min;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Program {
    registers: [usize; 3],
    instruction_pointer: usize,
    program: Vec<usize>,
}
impl Program {
    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => unreachable!(),
        }
    }
    pub fn clock_cycle(&mut self) -> Result<Option<usize>, ()> {
        if self.instruction_pointer >= self.program.len() - 1 {
            Err(())
        } else {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];
            self.instruction_pointer += 2;
            match opcode {
                0 => {
                    self.registers[0] /=
                        2_usize.pow(self.combo_operand(operand).try_into().unwrap())
                }
                1 => self.registers[1] ^= operand,
                2 => self.registers[1] = self.combo_operand(operand) % 8,
                3 => {
                    if self.registers[0] != 0 {
                        self.instruction_pointer = operand
                    }
                }
                4 => self.registers[1] ^= self.registers[2],
                5 => return Ok(Some(self.combo_operand(operand) % 8)),
                6 => {
                    self.registers[1] = self.registers[0]
                        / 2_usize.pow(self.combo_operand(operand).try_into().unwrap())
                }
                7 => {
                    self.registers[2] = self.registers[0]
                        / 2_usize.pow(self.combo_operand(operand).try_into().unwrap())
                }
                _ => unreachable!(),
            };
            Ok(None)
        }
    }
    pub fn output(mut self) -> Vec<usize> {
        let mut program_1_output = Vec::new();
        while let Ok(output) = self.clock_cycle() {
            if let Some(output) = output {
                program_1_output.push(output);
            }
        }

        program_1_output
    }
    pub fn find_first_register_value_for_quine(self) -> usize {
        let rng: Arc<Mutex<StdRng>> = Arc::new(Mutex::new(SeedableRng::seed_from_u64(5)));
        let (sender, receiver) = mpsc::channel();
        let num_cores = thread::available_parallelism().unwrap().get();

        for _ in 0..num_cores {
            let sender = sender.clone();
            let rng = rng.clone();
            let program = self.clone();
            let target_program = self.program.clone();
            let program_with_register_a = move |a: usize| {
                let mut new_program = program.clone();
                new_program.registers[0] = a;
                new_program.output()
            };
            thread::spawn(move || loop {
                let mut parent = {
                    let init_parent = rng.lock().unwrap().gen::<usize>();
                    let parent_output = program_with_register_a(init_parent);
                    (
                        init_parent,
                        output_distance(&parent_output, &target_program),
                    )
                };
                loop {
                    let mut best_child = parent;
                    let double_bit_flip_children =
                        (0..64)
                            .combinations_with_replacement(3)
                            .map(|flipped_bits| {
                                let mask = 2_usize.pow(flipped_bits[0])
                                    | 2_usize.pow(flipped_bits[1])
                                    | 2_usize.pow(flipped_bits[2]);
                                parent.0 ^ mask
                            });
                    for child in double_bit_flip_children {
                        let child_output = program_with_register_a(child);
                        let child_distance = output_distance(&child_output, &target_program);
                        if child_distance < best_child.1
                            || (child < best_child.0 && child_distance == best_child.1)
                        {
                            best_child = (child, child_distance);
                        }
                    }
                    if best_child.0 != parent.0 {
                        parent = best_child;
                    } else {
                        break;
                    }
                }
                sender.send(parent).unwrap();
            });
        }

        let mut best_overall = (0, usize::MAX);

        while best_overall.1 != 0 {
            let best_child = receiver.recv().unwrap();

            if best_child.1 < best_overall.1 {
                println!(
                    "Found better register {} with difference {}",
                    best_child.0, best_child.1
                );
                best_overall = best_child;
            } else {
                println!("Tried and failed with {}", best_child.0);
            }
        }

        best_overall.0
    }
}

fn output_distance(a: &[usize], b: &[usize]) -> usize {
    let len_a = a.len();
    let len_b = b.len();
    let mut dp = vec![0; (len_a + 1) * (len_b + 1)];

    for i in 0..=len_a {
        for j in 0..=len_b {
            if i == 0 {
                dp[i * (len_b + 1) + j] = j * 8;
            } else if j == 0 {
                dp[i * (len_b + 1) + j] = i * 8;
            } else if a[i - 1] == b[j - 1] {
                dp[i * (len_b + 1) + j] = dp[(i - 1) * (len_b + 1) + j - 1];
            } else {
                let cost = (a[i - 1] as isize - b[j - 1] as isize).unsigned_abs();
                dp[i * (len_b + 1) + j] = min(
                    dp[(i - 1) * (len_b + 1) + j - 1] + cost,
                    min(
                        dp[(i - 1) * (len_b + 1) + j] + 8,
                        dp[i * (len_b + 1) + j - 1] + 8,
                    ),
                );
            }
        }
    }

    dp[len_a * (len_b + 1) + len_b]
}

#[cfg(test)]
mod other_tests {
    use super::*;

    #[test]
    fn test_output_distance() {
        assert_eq!(output_distance(&[1, 2, 3], &[1, 2, 3]), 0);
        assert_eq!(output_distance(&[1, 2, 3], &[1, 2, 4]), 1);
        assert_eq!(output_distance(&[1, 2, 3], &[1, 2]), 8);
        assert_eq!(output_distance(&[1, 2, 3], &[1, 2, 3, 4]), 8);
        assert_eq!(output_distance(&[1, 2, 3], &[4, 5, 6]), 9);
        assert_eq!(output_distance(&[1, 2, 3], &[1, 3, 2]), 2);
    }
}

fn get_program_output(input_file: &str) -> (String, usize) {
    let input = read_lines(input_file);

    let program = {
        let registers = input[0..=2]
            .iter()
            .map(|s| s.split_ascii_whitespace().nth(2).unwrap().parse().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();

        let program = input[4]
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|o| o.parse::<usize>().unwrap())
            .collect_vec();

        Program {
            registers,
            instruction_pointer: 0,
            program,
        }
    };

    let answer_1 = program.clone().output().into_iter().join(",");

    let answer_2 = program.find_first_register_value_for_quine();

    (answer_1, answer_2)
}

base_aoc!("5,7,3,0", 117440);
