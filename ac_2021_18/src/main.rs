#![allow(dead_code)]
use advent_of_code_util::{parse::read_lines, remove_first_and_last};

#[derive(Debug)]
enum SFNPairPart {
    Num(isize),
    Pair(SnailfishNumber),
}
impl SFNPairPart {
    fn from_str(string: &str) -> Self {
        if string.contains(',') {
            SFNPairPart::Pair(SnailfishNumber::from_str(string))
        } else {
            SFNPairPart::Num(string.parse().unwrap())
        }
    }
    fn magnitude(&self) -> isize {
        match self {
            SFNPairPart::Num(val) => *val,
            SFNPairPart::Pair(sfn) => sfn.magnitude(),
        }
    }
    fn try_explode(&mut self, depth: usize) -> ((Option<isize>, Option<isize>), bool) {
        // [[a,b],c] -> [0, b+c], return ((a, None), true)
        // [a,[b,c]] -> [a+b, 0], return ((None, c), true)
        match &mut *self {
            SFNPairPart::Pair(val) => val.try_explode(depth + 1),
            SFNPairPart::Num(_) => ((None, None), false),
        }
    }
}

fn split_by_toplevel_comma(string: &str) -> (String, String) {
    let mut current_depth = 0;
    let mut on_first = true;
    let mut first = String::new();
    let mut last = String::new();
    for c in string.chars() {
        match c {
            '[' => {
                current_depth += 1;
                if on_first {
                    first.push(c)
                } else {
                    last.push(c)
                }
            }
            ']' => {
                current_depth -= 1;
                if on_first {
                    first.push(c)
                } else {
                    last.push(c)
                }
            }
            ',' => {
                if current_depth == 0 {
                    on_first = false;
                } else {
                    if on_first {
                        first.push(c)
                    } else {
                        last.push(c)
                    }
                }
            }
            _ => {
                if on_first {
                    first.push(c)
                } else {
                    last.push(c)
                }
            }
        }
    }
    (first, last)
}

#[derive(Debug)]
struct SnailfishNumber {
    left: Box<SFNPairPart>,
    right: Box<SFNPairPart>,
}
impl SnailfishNumber {
    fn from_str(string: &str) -> Self {
        let string_without_brackets = remove_first_and_last(string);
        let (left_string, right_string) = split_by_toplevel_comma(&string_without_brackets);
        SnailfishNumber {
            left: Box::new(SFNPairPart::from_str(&left_string)),
            right: Box::new(SFNPairPart::from_str(&right_string)),
        }
    }

    fn add(self, other: Self) -> Self {
        SnailfishNumber {
            left: Box::new(SFNPairPart::Pair(self)),
            right: Box::new(SFNPairPart::Pair(other)),
        }
    }

    fn add_right(&mut self, num: isize) {
        match &mut *self.right {
            SFNPairPart::Pair(val) => val.add_right(num),
            SFNPairPart::Num(val) => self.right = Box::new(SFNPairPart::Num(*val + num)),
        }
    }
    fn add_left(&mut self, num: isize) {
        match &mut *self.left {
            SFNPairPart::Pair(val) => val.add_left(num),
            SFNPairPart::Num(val) => self.left = Box::new(SFNPairPart::Num(*val + num)),
        }
    }

    fn try_explode(&mut self, depth: usize) -> ((Option<isize>, Option<isize>), bool) {
        let mut result = self.left.try_explode(depth);
        if !result.1 {
            result = self.right.try_explode(depth);
        }
        result
    }

    fn try_split(&mut self) -> bool {
        let mut split_so_far = match &mut *self.left {
            SFNPairPart::Pair(val) => val.try_split(),
            SFNPairPart::Num(val) => {
                if *val >= 10 {
                    self.left = Box::new(SFNPairPart::Pair(SnailfishNumber {
                        left: Box::new(SFNPairPart::Num(*val / 2)),
                        right: Box::new(SFNPairPart::Num((*val + 1) / 2)),
                    }));
                    true
                } else {
                    false
                }
            }
        };
        split_so_far = split_so_far
            || match &mut *self.right {
                SFNPairPart::Pair(val) => val.try_split(),
                SFNPairPart::Num(val) => {
                    if *val >= 10 {
                        self.right = Box::new(SFNPairPart::Pair(SnailfishNumber {
                            left: Box::new(SFNPairPart::Num(*val / 2)),
                            right: Box::new(SFNPairPart::Num((*val + 1) / 2)),
                        }));
                        true
                    } else {
                        false
                    }
                }
            };
        split_so_far
    }

    fn reduce(&mut self) {
        let mut try_again = false;
        try_again = try_again || self.try_explode(1).1;
        try_again = try_again || self.try_split();
        if try_again {
            self.reduce();
        }
    }

    fn magnitude(&self) -> isize {
        (3 * self.left.magnitude()) + (2 * self.right.magnitude())
    }
}

fn get_program_output(input_file: &str) -> (isize, usize) {
    let mut input = read_lines(input_file)
        .into_iter()
        .map(|line| SnailfishNumber::from_str(&line));

    let mut first = input.next().unwrap();
    for sfn in input {
        first = first.add(sfn);
        first.reduce();
    }

    (first.magnitude(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 4140);
        assert_eq!(part_2_output, 0);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
