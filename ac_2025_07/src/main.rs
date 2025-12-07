use advent_of_code_util::{base_aoc, parse::read_lines_of_chars};

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);
    let start_x = input[0].iter().position(|c| *c == 'S').unwrap();

    let mut part_1 = 0;

    let mut lines_down = vec![0; input[0].len()];
    lines_down[start_x] = 1;
    for y in 1..input.len() {
        let mut new_lines_down = vec![0; input[0].len()];
        for (x, prev_line) in lines_down.iter().enumerate() {
            if *prev_line > 0 {
                match input[y][x] {
                    '.' => new_lines_down[x] += prev_line,
                    '^' => {
                        new_lines_down[x - 1] += prev_line;
                        new_lines_down[x + 1] += prev_line;
                        part_1 += 1;
                    }
                    _ => unimplemented!(),
                }
            }
        }
        lines_down = new_lines_down;
    }

    let part_2 = lines_down.iter().sum();

    (part_1, part_2)
}

base_aoc!(21, 40);
