use advent_of_code_util::{parse::read_lines, Coordinate};

struct OctopusGrid {
    data: [[usize; 10]; 10],
    flashed_this_step: [[bool; 10]; 10],
    flashes_this_step: usize,
}
impl OctopusGrid {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut new_octopus_grid = OctopusGrid {
            data: [[0; 10]; 10],
            flashed_this_step: [[false; 10]; 10],
            flashes_this_step: 0,
        };
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                new_octopus_grid.data[x][y] = char.to_digit(10).unwrap() as usize;
            }
        }
        new_octopus_grid
    }

    fn flash(&mut self, coord: Coordinate) {
        if !self.flashed_this_step[coord.x][coord.y] {
            self.flashed_this_step[coord.x][coord.y] = true;
            self.flashes_this_step += 1;
            coord
                .get_surrounding_coordinates(10, 10)
                .into_iter()
                .for_each(|new_coord| {
                    self.data[new_coord.x][new_coord.y] += 1;
                    if self.data[new_coord.x][new_coord.y] > 9 {
                        self.flash(new_coord);
                    }
                });
        }
    }

    fn step(&mut self) -> usize {
        for x in 0..10 {
            for y in 0..10 {
                self.data[x][y] += 1;
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                if self.data[x][y] > 9 {
                    self.flash(Coordinate { x, y });
                }
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                if self.data[x][y] > 9 {
                    self.data[x][y] = 0;
                    self.flashed_this_step[x][y] = false;
                }
            }
        }

        let fts = self.flashes_this_step;
        self.flashes_this_step = 0;
        fts
    }
    fn step_n(&mut self, n: usize) -> usize {
        let mut sum = 0;
        for _ in 0..n {
            sum += self.step();
        }
        sum
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let mut og_1 = OctopusGrid::from_lines(input.clone());

    let num_flashes = og_1.step_n(100);

    let mut og_2 = OctopusGrid::from_lines(input.clone());
    let mut counter = 0;
    let synchronized_flash = loop {
        counter += 1;
        if og_2.step() == 100 {
            break counter;
        }
    };

    (num_flashes, synchronized_flash)
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 1656);
        assert_eq!(part_2_output, 195);
    }
}
