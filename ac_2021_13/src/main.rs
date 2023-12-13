use advent_of_code_util::{parse::read_blocks, Coordinate};
use itertools::Itertools;

enum Direction {
    Horizontal,
    Vertical,
}
struct Fold {
    direction: Direction,
    val: usize,
}
impl Fold {
    fn from_str(string: &str) -> Self {
        string
            .split_whitespace()
            .last()
            .unwrap()
            .split('=')
            .collect_tuple::<(&str, &str)>()
            .map(|(dir_str, amt_str)| Fold {
                direction: match dir_str {
                    "x" => Direction::Horizontal,
                    "y" => Direction::Vertical,
                    _ => panic!(),
                },
                val: amt_str.parse().unwrap(),
            })
            .unwrap()
    }
}

struct Paper {
    dots: Vec<Coordinate>,
}
impl Paper {
    fn fold(&mut self, f: &Fold) {
        self.dots = match f.direction {
            Direction::Horizontal => self
                .dots
                .clone()
                .into_iter()
                .map(|dot| {
                    if dot.x > f.val {
                        Coordinate {
                            x: (2 * f.val) - dot.x,
                            y: dot.y,
                        }
                    } else {
                        dot
                    }
                })
                .unique()
                .collect(),
            Direction::Vertical => self
                .dots
                .clone()
                .into_iter()
                .map(|dot| {
                    if dot.y > f.val {
                        Coordinate {
                            x: dot.x,
                            y: (2 * f.val) - dot.y,
                        }
                    } else {
                        dot
                    }
                })
                .unique()
                .collect(),
        }
    }
    fn num_visible_dots(&self) -> usize {
        self.dots.len()
    }
}
impl ToString for Paper {
    fn to_string(&self) -> String {
        let max_x = self.dots.iter().map(|dot| dot.x).max().unwrap();
        let max_y = self.dots.iter().map(|dot| dot.y).max().unwrap();
        let mut return_string = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.iter().any(|dot| dot.x == x && dot.y == y) {
                    return_string.push('#');
                } else {
                    return_string.push('.');
                }
            }
            return_string.push('\n');
        }
        return_string
    }
}

fn get_program_output(input_file: &str) -> (usize, &str) {
    let input = read_blocks(input_file);
    let coordinates = input[0]
        .iter()
        .map(|coord_string| coord_string.parse().unwrap())
        .collect();
    let mut folds = input[1]
        .iter()
        .map(|fold_string| Fold::from_str(fold_string));

    let mut paper = Paper { dots: coordinates };

    paper.fold(&folds.next().unwrap());

    let after_first_fold = paper.num_visible_dots();

    folds.for_each(|fold| paper.fold(&fold));

    println!("{}", paper.to_string());

    (after_first_fold, "HZLEHJRK")
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
        assert_eq!(part_1_output, 17);
        assert_eq!(part_2_output, "HZLEHJRK");
    }
}
