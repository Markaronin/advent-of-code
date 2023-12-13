use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::Itertools;

#[derive(Debug)]
struct MapRange {
    pub from_start: usize,
    pub to_start: usize,
    pub len: usize,
}
impl MapRange {
    pub fn from_line(line: &str) -> Self {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap());
        Self {
            to_start: nums.next().unwrap(),
            from_start: nums.next().unwrap(),
            len: nums.next().unwrap(),
        }
    }
    pub fn map(&self, from: usize) -> Option<usize> {
        if (self.from_start..=self.from_start + self.len).contains(&from) {
            Some(self.to_start + (from - self.from_start))
        } else {
            None
        }
    }
}

fn get_min_seed_location(seeds: &Vec<usize>, maps: &Vec<Vec<MapRange>>) -> usize {
    seeds
        .iter()
        .map(|seed| {
            let mut new_seed = *seed;
            for map in maps {
                new_seed = map
                    .iter()
                    .find_map(|map_range| map_range.map(new_seed))
                    .unwrap_or(new_seed);
            }
            if new_seed % 1000000 == 0 {
                println!("{new_seed}");
            }
            new_seed
        })
        .min()
        .unwrap()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_blocks(input_file).into_iter();

    let seeds = input.next().unwrap()[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let maps = input
        .map(|raw| {
            raw.into_iter()
                .skip(1)
                .map(|line| MapRange::from_line(&line))
                .sorted_by_key(|m| m.from_start)
                .collect_vec()
        })
        .collect_vec();

    let result_1 = get_min_seed_location(&seeds, &maps);

    let seeds_2 = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .flat_map(|mut pair| {
            let start = *pair.next().unwrap();
            let amt = *pair.next().unwrap();
            start..start + amt
        })
        .collect_vec();

    let result_2 = get_min_seed_location(&seeds_2, &maps);

    (result_1, result_2 - 1)
}

base_aoc!(35, 46);
