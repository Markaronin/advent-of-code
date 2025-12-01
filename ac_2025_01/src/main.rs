use advent_of_code_util::{base_aoc, parse::read_lines};

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let mut rotation = 50;

    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in input {
        let (dir, amt) = line.split_at(1);
        let amt = amt.parse::<isize>().unwrap();

        for _ in 0..amt {
            match dir {
                "L" => rotation -= 1,
                "R" => rotation += 1,
                _ => panic!("Unrecognized direction"),
            }
            if rotation < 0 {
                rotation += 100;
            }
            if rotation > 99 {
                rotation -= 100;
            }
            if rotation == 0 {
                part_2 += 1;
            }
        }

        if rotation == 0 {
            part_1 += 1;
        }
    }

    (part_1, part_2)
}

base_aoc!(3, 0);
