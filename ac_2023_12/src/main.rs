use std::collections::{BTreeMap, VecDeque};

use advent_of_code_util::*;
use itertools::Itertools;

// #[derive(Debug, Clone)]
// struct Spring {
//     pattern: String,
//     constraints: Vec<usize>,
// }
// impl Spring {
//     pub fn from_line(line: &str) -> Self {
//         let (pattern, raw_constraints) = line.split_ascii_whitespace().collect_tuple().unwrap();
//         let constraints = raw_constraints
//             .split(",")
//             .map(|c| c.parse::<usize>().unwrap())
//             .collect_vec();

//         Self {
//             pattern: pattern.to_string(),
//             constraints,
//         }
//     }
//     pub fn unfold(self) -> Self {
//         Self {
//             pattern: vec![self.pattern; 5].join("?"),
//             constraints: self.constraints.repeat(5),
//         }
//     }
//     pub fn has_unknowns(&self) -> bool {
//         self.pattern.contains("?")
//     }
//     pub fn get_variants(&self) -> Vec<Self> {
//         let variants = vec![
//             Self {
//                 pattern: self.pattern.replacen("?", ".", 1),
//                 constraints: self.constraints.clone(),
//             },
//             Self {
//                 pattern: self.pattern.replacen("?", "#", 1),
//                 constraints: self.constraints.clone(),
//             },
//         ];

//         todo!()
//     }
//     pub fn meets_constraints(&self) -> bool {
//         if self.has_unknowns() {
//             let beginning = self
//                 .pattern
//                 .split(".")
//                 .filter(|m| !m.is_empty())
//                 .take_while(|m| !m.contains("?"))
//                 .map(|m| m.len())
//                 .collect_vec();
//             let mut end = self
//                 .pattern
//                 .split(".")
//                 .filter(|m| !m.is_empty())
//                 .collect_vec()
//                 .into_iter()
//                 .rev()
//                 .take_while(|m| !m.contains("?"))
//                 .map(|m| m.len())
//                 .collect_vec();
//             end.reverse();

//             dbg!(self, &beginning, &end);

//             self.constraints.starts_with(&beginning) && self.constraints.ends_with(&end)
//         } else {
//             self.pattern
//                 .split(".")
//                 .filter(|m| !m.is_empty())
//                 .map(|m| m.len())
//                 .collect_vec()
//                 == self.constraints
//         }
//     }
//     pub fn num_variants_meeting_constraints(self) -> usize {
//         let mut var_queue = vec![self];
//         let mut num_variants = 0;

//         let mut checks = 0;

//         while let Some(next) = var_queue.pop() {
//             if next.has_unknowns() {
//                 var_queue.append(&mut next.get_variants())
//             } else {
//                 num_variants += 1;
//             }
//             checks += 1;
//         }

//         dbg!(checks, num_variants);
//         panic!();

//         num_variants
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Spring2 {
    pattern: VecDeque<String>,
    constraints: VecDeque<String>,
}
impl Spring2 {
    pub fn from_line(line: &str) -> Option<Self> {
        // initial parse filters out empty strings, chomps segments until the first one with a question mark (just returning 0 if it doesn't meet constraints)
        let (raw_pattern, raw_constraints) = line.split_ascii_whitespace().collect_tuple().unwrap();

        let pattern = raw_pattern
            .split(".")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let constraints = raw_constraints
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .map(|amt| "#".repeat(amt))
            .collect();

        let unchecked = Self {
            pattern,
            constraints,
        };

        unchecked.process_unchecked_variant()
    }

    fn create_unchecked_variants(&self) -> Vec<Self> {
        assert!(self.pattern.len() > 0);
        assert!(self.pattern[0].contains("?"));

        let mut results = vec![];

        for c in [".", "#"] {
            let mut new = self.clone();
            new.pattern[0] = new.pattern[0].replacen('?', c, 1);
            results.push(new);
        }

        results
    }

    fn process_unchecked_variant(mut self) -> Option<Self> {
        let mut first = self
            .pattern
            .pop_front()
            .unwrap()
            .split(".")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect_vec();
        first.reverse();
        for i in first {
            self.pattern.push_front(i);
        }

        while self.pattern.len() > 0 && self.constraints.len() > 0 && !self.pattern[0].contains("?")
        {
            if self.pattern[0] != self.constraints[0] {
                return None;
            } else {
                self.pattern.pop_front().unwrap();
                self.constraints.pop_front().unwrap();
            }
        }
        if self.pattern.len() > 0 {
            // there is at least some pattern left
            if self.constraints.len() > 0 {
                // there are some of both left
                if self.pattern[0].chars().take_while(|c| *c == '#').count()
                    <= self.constraints[0].len()
                {
                    Some(self)
                } else {
                    None
                }
            } else {
                // some pattern, no more constraints
                if !self.pattern[0].contains("#") {
                    // If it's all question marks
                    Some(self)
                } else {
                    None
                }
            }
        } else if self.constraints.len() > 0 {
            // no more chars but there are constraints
            None
        } else {
            // no more pattern or constraints
            Some(self)
        }
    }

    pub fn num_variants_meeting_constraints(&self, cache: &mut BTreeMap<Spring2, usize>) -> usize {
        if self.constraints == self.pattern {
            return 1;
        }
        match cache.get(&self) {
            Some(val) => *val,
            None => {
                let mut sum = 0;
                for v in self
                    .create_unchecked_variants()
                    .into_iter()
                    .flat_map(|v| v.process_unchecked_variant())
                {
                    sum += v.num_variants_meeting_constraints(cache);
                }
                cache.insert(self.clone(), sum);
                sum
            }
        }
    }
}

fn unfold_line(line: &str) -> String {
    let (raw_pattern, raw_constraints) = line.split_ascii_whitespace().collect_tuple().unwrap();

    vec![raw_pattern.to_string(); 5].join("?")
        + " "
        + &vec![raw_constraints.to_string(); 5].join(",")
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let result_1 = input
        .clone()
        .into_iter()
        .flat_map(|line| Spring2::from_line(&line))
        .map(|spring| spring.num_variants_meeting_constraints(&mut BTreeMap::new()))
        .sum::<usize>();

    let result_2 = input
        .clone()
        .into_iter()
        .map(|line| unfold_line(&line))
        .flat_map(|line| Spring2::from_line(&line))
        .map(|spring| spring.num_variants_meeting_constraints(&mut BTreeMap::new()))
        .sum::<usize>();

    (result_1, result_2)
}

base_aoc!(21, 525152);
