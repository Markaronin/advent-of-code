use std::{collections::BTreeMap, str::FromStr};

use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::Itertools;

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

#[derive(Debug, Clone)]
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

    (result_1, 0)
}

base_aoc!(19114, 0);
