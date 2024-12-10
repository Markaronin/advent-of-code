use std::mem::replace;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn compress_blocks(input: &[usize]) -> usize {
    let mut fs: Vec<Option<usize>> = Vec::new();
    for (id, sizes) in input.chunks(2).enumerate() {
        let block_size = sizes[0];
        let empty_size = sizes.get(1);
        fs.extend(vec![Some(id); block_size]);
        if let Some(empty_size) = empty_size {
            fs.extend(vec![None; *empty_size]);
        }
    }

    let mut front_ptr = 0;
    let mut end_ptr = fs.len() - 1;
    while front_ptr != end_ptr {
        if fs[front_ptr].is_some() {
            front_ptr += 1;
        } else if fs[end_ptr].is_none() {
            end_ptr -= 1;
        } else {
            fs.swap(front_ptr, end_ptr);
        }
    }

    fs.into_iter()
        .flatten()
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

enum FileSystem {
    File { id: usize, size: usize },
    Empty { size: usize },
}
impl FileSystem {
    pub fn from_input(input: &[usize]) -> Vec<Self> {
        let mut fs: Vec<FileSystem> = Vec::new();
        for (id, sizes) in input.chunks(2).enumerate() {
            let block_size = sizes[0];
            let empty_size = sizes.get(1);
            fs.push(FileSystem::File {
                id,
                size: block_size,
            });
            if let Some(empty_size) = empty_size {
                fs.push(FileSystem::Empty { size: *empty_size });
            }
        }

        fs
    }
    pub fn calc_checksum(&self, starting_id: usize) -> usize {
        match self {
            FileSystem::File { id, size } => (starting_id..starting_id + size)
                .map(|index: usize| id * index)
                .sum(),
            FileSystem::Empty { size: _ } => 0,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            FileSystem::File { id: _, size } => *size,
            FileSystem::Empty { size } => *size,
        }
    }
    pub fn set_size(&mut self, new_size: usize) {
        match self {
            FileSystem::File { id: _, size } => *size = new_size,
            FileSystem::Empty { size } => *size = new_size,
        }
    }
}

fn compress_files(input: &[usize]) -> usize {
    let mut fs = FileSystem::from_input(input);

    let mut end_ptr = fs.len() - 1;
    loop {
        if let FileSystem::File { id: _, size } = fs[end_ptr] {
            let mut front_ptr = 0;
            while front_ptr < end_ptr {
                if let FileSystem::Empty { size: empty_size } = fs[front_ptr] {
                    match empty_size.cmp(&size) {
                        std::cmp::Ordering::Less => {}
                        std::cmp::Ordering::Equal => {
                            fs.swap(front_ptr, end_ptr);
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            let old_file = replace(&mut fs[end_ptr], FileSystem::Empty { size });
                            fs.insert(front_ptr, old_file);
                            fs[front_ptr + 1].set_size(empty_size - size);
                            end_ptr += 1;
                            break;
                        }
                    }
                }

                front_ptr += 1;
            }
        }

        if end_ptr == 0 {
            break;
        }
        end_ptr -= 1;
    }

    let mut fs_index = 0;
    let mut checksum = 0;
    for f in fs {
        checksum += f.calc_checksum(fs_index);
        fs_index += f.size();
    }
    checksum
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let answer_1 = compress_blocks(&input);
    let answer_2 = compress_files(&input);

    (answer_1, answer_2)
}

base_aoc!(1928, 2858);
