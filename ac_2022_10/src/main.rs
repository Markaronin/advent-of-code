use advent_of_code_util::*;
use itertools::Itertools;

enum Instruction {
    Noop,
    Addx(isize),
}
impl Instruction {
    pub fn from_line(line: &str) -> Self {
        if line == "noop" {
            Self::Noop
        } else if line.starts_with("addx") {
            let val = line
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<isize>()
                .unwrap();
            Self::Addx(val)
        } else {
            panic!("Invalid instruction")
        }
    }
}

fn get_program_output(input_file: &str) -> (isize, String) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| Instruction::from_line(&line))
        .collect_vec();

    let mut cycles: isize = 0;
    let mut register_x: isize = 1;
    let mut result_1: isize = 0;
    let mut image = [[false; 40]; 6];

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                // Image stuff
                let x = (cycles % 40) as usize;
                let y = (cycles / 40) as usize;
                let draw = abs_diff(x, register_x as usize) <= 1;
                if draw {
                    image[y][x] = true;
                }
                // End Image stuff
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    result_1 += cycles * register_x;
                }
            }
            Instruction::Addx(val) => {
                // Image stuff
                let x = (cycles % 40) as usize;
                let y = (cycles / 40) as usize;
                let draw = abs_diff(x, register_x as usize) <= 1;
                if draw {
                    image[y][x] = true;
                }
                // End Image stuff
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    result_1 += cycles * register_x;
                }
                // Image stuff
                let x = (cycles % 40) as usize;
                let y = (cycles / 40) as usize;
                let draw = abs_diff(x, register_x as usize) <= 1;
                if draw {
                    image[y][x] = true;
                }
                // End Image stuff
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    result_1 += cycles * register_x;
                }
                register_x += val;
            }
        };
    }

    let printed_image = image
        .map(|row| row.map(|col| if col { "#" } else { "." }).join(""))
        .join("\n");

    (result_1, printed_image)
}

base_aoc!(
    13140,
    r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
);
