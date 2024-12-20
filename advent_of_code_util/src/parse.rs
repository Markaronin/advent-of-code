use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use itertools::Itertools;

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn read_parsed_lines<T, P>(filename: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| T::from_str(&line).unwrap())
        .collect()
}

pub fn read_lines_of_chars<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect_vec())
        .collect()
}

pub fn read_grid_of_digits<P>(filename: P) -> Vec<Vec<usize>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect()
}

pub fn read_list_of_lists<P, F, T>(filename: P, separator: &str, transform: F) -> Vec<Vec<T>>
where
    P: AsRef<Path>,
    F: FnMut(&str) -> T + Copy,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(separator).map(transform).collect_vec())
        .collect()
}

pub fn read_blocks<P>(filename: P) -> Vec<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let mut blocks = vec![];
    let mut latest_block = vec![];
    for line in io::BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            blocks.push(latest_block);
            latest_block = vec![];
        } else {
            latest_block.push(line);
        }
    }
    if !latest_block.is_empty() {
        blocks.push(latest_block);
    }
    blocks
}

pub fn split_block_on_whitespace(block: Vec<String>) -> Vec<String> {
    block
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(|split_line| split_line.to_string())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use crate::parse::*;

    #[test]
    fn split_block_on_whitespace_test() {
        assert_eq!(
            split_block_on_whitespace(vec![
                "pid:161cm eyr:2025 hcl:#b6652a".to_string(),
                "cid:213".to_string(),
                "ecl:xry".to_string(),
                "hgt:150cm".to_string(),
                "iyr:2024 byr:2012".to_string()
            ]),
            vec![
                "pid:161cm".to_string(),
                "eyr:2025".to_string(),
                "hcl:#b6652a".to_string(),
                "cid:213".to_string(),
                "ecl:xry".to_string(),
                "hgt:150cm".to_string(),
                "iyr:2024".to_string(),
                "byr:2012".to_string()
            ]
        );
    }
}
