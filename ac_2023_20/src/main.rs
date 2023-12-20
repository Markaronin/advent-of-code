use std::collections::{BTreeMap, VecDeque};

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        outputs: Vec<String>,
    },
    FlipFlop {
        on: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        input_values: BTreeMap<String, Pulse>,
        outputs: Vec<String>,
    },
}

fn fill_conjunction_maps(input: &mut BTreeMap<String, Module>) {
    let output_input_pairs = input
        .iter()
        .flat_map(|(label, module)| {
            let outputs = match module {
                Module::Broadcaster { outputs } => outputs,
                Module::FlipFlop { outputs, .. } => outputs,
                Module::Conjunction { outputs, .. } => outputs,
            };
            outputs.iter().map(|output| (label.clone(), output.clone()))
        })
        .filter(|(_, to_label)| to_label != "output")
        .collect_vec();

    for (from_label, to_label) in output_input_pairs {
        if let Some(Module::Conjunction { input_values, .. }) = input.get_mut(&to_label) {
            input_values.insert(from_label, Pulse::Low);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

fn calculate_multiplied_impulse_score(mut input: BTreeMap<String, Module>) -> usize {
    let mut num_low_pulses = 0;
    let mut num_high_pulses = 0;

    for _ in 1..=1000 {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        while let Some((from_label, pulse, to_label)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::Low => num_low_pulses += 1,
                Pulse::High => num_high_pulses += 1,
            };

            match input.get_mut(&to_label) {
                None => {
                    // Do nothing
                }
                Some(Module::Broadcaster { outputs }) => {
                    for output in outputs {
                        pulse_queue.push_back((to_label.clone(), pulse, output.clone()))
                    }
                }
                Some(Module::FlipFlop { on, outputs }) => {
                    if pulse == Pulse::Low {
                        let new_pulse = match on {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        *on = !*on;
                        for output in outputs {
                            pulse_queue.push_back((to_label.clone(), new_pulse, output.clone()))
                        }
                    }
                }
                Some(Module::Conjunction {
                    input_values,
                    outputs,
                }) => {
                    *input_values.get_mut(&from_label).unwrap() = pulse;
                    let send_pulse = if input_values.values().all(|p| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for output in outputs {
                        pulse_queue.push_back((to_label.clone(), send_pulse, output.clone()))
                    }
                }
            }
        }
    }

    num_low_pulses * num_high_pulses
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            let (raw_label, raw_outputs) = line.split(" -> ").collect_tuple().unwrap();
            let outputs = raw_outputs.split(", ").map(|o| o.to_string()).collect_vec();
            match raw_label.chars().next().unwrap() {
                'b' => ("broadcaster".to_string(), Module::Broadcaster { outputs }),
                '%' => (
                    raw_label[1..].to_string(),
                    Module::FlipFlop { on: false, outputs },
                ),
                '&' => (
                    raw_label[1..].to_string(),
                    Module::Conjunction {
                        input_values: BTreeMap::new(),
                        outputs,
                    },
                ),
                _ => unreachable!(),
            }
        })
        .collect::<BTreeMap<_, _>>();
    fill_conjunction_maps(&mut input);

    let result_1 = calculate_multiplied_impulse_score(input.clone());

    (result_1, 0)
}

base_aoc!(11687500, 0);
