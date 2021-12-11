use std::str::Chars;

use advent_of_code_util::*;

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    col: usize,
}
impl BoardingPass {
    fn from_str(string: &str) -> Self {
        fn binary_search(
            mut min: usize,
            mut max: usize,
            chars: Chars,
            top_char: char,
            bottom_char: char,
        ) -> usize {
            for identifier in chars {
                let middle = min + ((max - min) / 2);
                match identifier {
                    c if c == top_char => min = middle + 1,
                    c if c == bottom_char => max = middle,
                    _ => panic!(),
                }
            }
            min
        }

        let (row_identifier, col_identifier) = string.split_at(7);

        let new_boarding_pass = BoardingPass {
            row: binary_search(0, 127, row_identifier.chars(), 'B', 'F'),
            col: binary_search(0, 7, col_identifier.chars(), 'R', 'L'),
        };
        new_boarding_pass
    }
    fn seat_id(&self) -> usize {
        (self.row * 8) + self.col
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut boarding_passes = read_lines(input_file)
        .into_iter()
        .map(|line| BoardingPass::from_str(&line))
        .collect::<Vec<BoardingPass>>();
    boarding_passes.sort_unstable_by(|a, b| a.seat_id().cmp(&b.seat_id()));
    let mut possibly_my_seat = None;
    for i in 0..=boarding_passes.len() - 2 {
        if boarding_passes[i].seat_id() + 1 != boarding_passes[i + 1].seat_id() {
            possibly_my_seat = Some(boarding_passes[i].seat_id() + 1);
        }
    }
    let my_seat = possibly_my_seat.unwrap();
    (
        boarding_passes
            .iter()
            .map(|boarding_pass| boarding_pass.seat_id())
            .max()
            .unwrap(),
        my_seat,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, _) = get_program_output(&file_path);
        assert_eq!(part_1_output, 820);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
