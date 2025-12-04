use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_lines_of_chars(input_file);

    let mut part_1 = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let coord = Coordinate { x, y };
            let surrounding = coord.get_surrounding_coordinates(input[y].len(), input.len());
            let is_accessible = surrounding
                .into_iter()
                .filter(|c| input[c.y][c.x] == '@')
                .count()
                < 4;
            if input[y][x] == '@' && is_accessible {
                part_1 += 1;
            }
        }
    }

    let mut part_2 = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                let coord = Coordinate { x, y };
                let surrounding = coord.get_surrounding_coordinates(input[y].len(), input.len());
                let is_accessible = surrounding
                    .into_iter()
                    .filter(|c| input[c.y][c.x] == '@')
                    .count()
                    < 4;
                if input[y][x] == '@' && is_accessible {
                    input[y][x] = '.';
                    part_2 += 1;
                    removed = true;
                }
            }
        }
    }

    (part_1, part_2)
}

base_aoc!(13, 43);
