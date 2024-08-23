use std::{collections::BTreeMap, ops::RangeInclusive, str::FromStr};

use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::Itertools;

type PartRanges = [RangeInclusive<usize>; 4];
fn amount(ranges: &PartRanges) -> usize {
    ranges.iter().map(|r| r.size_hint().0).product()
}

#[derive(Debug)]
struct Part {
    ratings: [usize; 4],
}
impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ratings = s[1..s.len() - 1]
            .split(',')
            .map(|v| v.split('=').nth(1).unwrap().parse::<usize>().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        Ok(Self { ratings })
    }
}
impl Part {
    pub fn score(&self) -> usize {
        self.ratings.iter().sum::<usize>()
    }
    pub fn rating(&self, rating_label: char) -> usize {
        match rating_label {
            'x' => self.ratings[0],
            'm' => self.ratings[1],
            'a' => self.ratings[2],
            's' => self.ratings[3],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Destination {
    Accepted,
    Rejected,
    Rule(String),
}
impl FromStr for Destination {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            val => Self::Rule(val.to_string()),
        })
    }
}
#[derive(Debug)]
enum LessOrGreater {
    Less,
    Greater,
}
#[derive(Debug)]
struct Rule {
    rating: char,
    condition: LessOrGreater,
    value: usize,
    destination: Destination,
}
impl Rule {
    pub fn matches(&self, part: &Part) -> Option<Destination> {
        if match self.condition {
            LessOrGreater::Less => part.rating(self.rating) < self.value,
            LessOrGreater::Greater => part.rating(self.rating) > self.value,
        } {
            Some(self.destination.clone())
        } else {
            None
        }
    }

    pub fn split_ranges(&self, ranges: &PartRanges) -> (Option<PartRanges>, Option<PartRanges>) {
        let range_index = match self.rating {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        let (split, remainder) = match self.condition {
            LessOrGreater::Less => (
                *ranges[range_index].start()..=self.value - 1,
                self.value..=*ranges[range_index].end(),
            ),
            LessOrGreater::Greater => (
                self.value + 1..=*ranges[range_index].end(),
                *ranges[range_index].start()..=self.value,
            ),
        };
        (
            if split.is_empty() {
                None
            } else {
                let mut s = ranges.clone();
                s[range_index] = split;
                Some(s)
            },
            if remainder.is_empty() {
                None
            } else {
                let mut r = ranges.clone();
                r[range_index] = remainder;
                Some(r)
            },
        )
    }
}
impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_rule, raw_destination) = s.split(':').collect_tuple().unwrap();
        let mut raw_rule = raw_rule.chars();
        let rating = raw_rule.next().unwrap();
        let condition = match raw_rule.next().unwrap() {
            '<' => LessOrGreater::Less,
            '>' => LessOrGreater::Greater,
            _ => unreachable!(),
        };
        let value = raw_rule.join("").parse::<usize>().unwrap();
        let destination = Destination::from_str(raw_destination).unwrap();
        Ok(Self {
            rating,
            condition,
            value,
            destination,
        })
    }
}
#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    backup: Destination,
}
impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',').collect_vec();
        let backup = Destination::from_str(s.pop().unwrap()).unwrap();
        let rules = s
            .iter()
            .map(|raw| Rule::from_str(raw).unwrap())
            .collect_vec();

        Ok(Self { rules, backup })
    }
}
impl Workflow {
    pub fn process(workflows: &BTreeMap<String, Workflow>, part: &Part) -> Destination {
        let mut destination = Destination::Rule("in".to_string());
        while let Destination::Rule(workflow_label) = &destination {
            let current_workflow = workflows.get(workflow_label).unwrap();
            destination = match current_workflow
                .rules
                .iter()
                .find_map(|rule| rule.matches(part))
            {
                Some(dest) => dest,
                None => current_workflow.backup.clone(),
            };
        }
        destination
    }

    pub fn process_ranges(
        label: Destination,
        workflows: &BTreeMap<String, Workflow>,
        ranges: PartRanges,
    ) -> usize {
        match label {
            Destination::Accepted => amount(&ranges),
            Destination::Rejected => 0,
            Destination::Rule(label) => {
                let current_workflow = workflows.get(&label).unwrap();
                let mut remainder = Some(ranges.clone());
                let mut total_sum = 0;
                for rule in &current_workflow.rules {
                    if remainder.is_some() {
                        let result = rule.split_ranges(&remainder.unwrap());
                        if let Some(split) = result.0 {
                            total_sum += Workflow::process_ranges(
                                rule.destination.clone(),
                                workflows,
                                split,
                            );
                        }
                        remainder = result.1
                    }
                }
                if let Some(remainder) = remainder {
                    total_sum += Workflow::process_ranges(
                        current_workflow.backup.clone(),
                        workflows,
                        remainder,
                    );
                }

                total_sum
            }
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file);

    let workflows: BTreeMap<String, Workflow> = input[0]
        .iter()
        .map(|line| {
            let (raw_label, raw_workflow) = line.split('{').collect_tuple().unwrap();
            (
                raw_label.to_string(),
                Workflow::from_str(&raw_workflow[..raw_workflow.len() - 1]).unwrap(),
            )
        })
        .collect();

    let parts: Vec<Part> = input[1]
        .iter()
        .map(|line| Part::from_str(line).unwrap())
        .collect();

    let result_1 = parts
        .iter()
        .filter_map(|part| match Workflow::process(&workflows, part) {
            Destination::Accepted => Some(part.score()),
            Destination::Rejected => None,
            Destination::Rule(_) => unreachable!(),
        })
        .sum();

    let result_2 = Workflow::process_ranges(
        Destination::Rule("in".to_string()),
        &workflows,
        [1..=4000, 1..=4000, 1..=4000, 1..=4000],
    );

    (result_1, result_2)
}

base_aoc!(19114, 167409079868000);
