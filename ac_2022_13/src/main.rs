use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::{zip, Itertools};
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Val(usize),
    Arr(Vec<Packet>),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Val(v1), Self::Val(v2)) => v1.cmp(v2),
            (Self::Val(v1), Self::Arr(v2)) => {
                Self::Arr(vec![Self::Val(*v1)]).cmp(&Self::Arr(v2.clone()))
            }
            (Self::Arr(v1), Self::Val(v2)) => {
                Self::Arr(v1.clone()).cmp(&Self::Arr(vec![Self::Val(*v2)]))
            }
            (Self::Arr(v1), Self::Arr(v2)) => {
                for (p1, p2) in zip(v1, v2) {
                    if p1 != p2 {
                        return p1.cmp(p2);
                    }
                }

                v1.len().cmp(&v2.len())
            }
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file)
        .into_iter()
        .map(|block| {
            block
                .into_iter()
                .map(|line| serde_json::from_str::<Packet>(&line).unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();

    let result_1 = input
        .iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1 <= p2)
        .map(|(i, _)| i + 1)
        .sum();

    let divider_packets = vec![Packet::Val(2), Packet::Val(6)];

    let result_2 = input
        .into_iter()
        .flat_map(|(p1, p2)| vec![p1, p2])
        .chain(divider_packets.clone())
        .rev() // Reverse to put divider packets at the start
        .sorted()
        .enumerate()
        .filter_map(|(i, packet)| {
            println!("{packet:?}");
            if divider_packets.contains(&packet) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();

    (result_1, result_2)
}

base_aoc!(13, 140);
