use std::{collections::BTreeMap, fs::read_to_string, str::FromStr};

use advent_of_code_util::base_aoc;
use itertools::Itertools;

struct ServerRack {
    values: BTreeMap<String, Vec<String>>,
}
impl ServerRack {
    fn num_paths_between(
        &self,
        start: &str,
        end: &str,
        mut required_visits: BTreeMap<String, bool>,
        cache: &mut BTreeMap<(String, String, BTreeMap<String, bool>), usize>,
    ) -> usize {
        if start == end {
            if required_visits.values().all(|v| *v) {
                1
            } else {
                0
            }
        } else if let Some(v) =
            cache.get(&(start.to_string(), end.to_string(), required_visits.clone()))
        {
            *v
        } else {
            if let Some(visit) = required_visits.get_mut(start) {
                *visit = true;
            }
            let outputs = self.values.get(start).unwrap();
            let v = outputs
                .iter()
                .map(|o| self.num_paths_between(o, end, required_visits.clone(), cache))
                .sum();
            cache.insert((start.to_string(), end.to_string(), required_visits), v);
            v
        }
    }

    pub fn num_paths_from_you_to_out(&self) -> usize {
        let mut cache = BTreeMap::new();

        self.num_paths_between("you", "out", BTreeMap::new(), &mut cache)
    }

    pub fn num_paths_from_svr_to_out(&self) -> usize {
        let mut cache = BTreeMap::new();

        let mut required_visits = BTreeMap::new();
        required_visits.insert("fft".to_string(), false);
        required_visits.insert("dac".to_string(), false);

        self.num_paths_between("svr", "out", required_visits, &mut cache)
    }
}
impl FromStr for ServerRack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s
                .lines()
                .map(|line| {
                    let raw = line
                        .split(": ")
                        .map(|a| a.to_owned())
                        .collect_tuple::<(String, String)>()
                        .unwrap();

                    (raw.0, raw.1.split(' ').map(|a| a.to_owned()).collect_vec())
                })
                .collect(),
        })
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = ServerRack::from_str(&read_to_string(input_file).unwrap()).unwrap();

    let part_1 = input.num_paths_from_you_to_out();
    let part_2 = input.num_paths_from_svr_to_out();

    (part_1, part_2)
}

base_aoc!(8, 2);
