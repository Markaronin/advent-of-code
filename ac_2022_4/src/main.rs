use advent_of_code_util::{base_aoc, parse::read_lines};

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn from_str(s: &str) -> Self {
        let mut parsed = s.split('-').map(|n| n.parse::<usize>().unwrap());
        Range {
            start: parsed.next().unwrap(),
            end: parsed.next().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (other.start <= self.start && other.end >= self.start)
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            let mut ranges = line.split(',').map(Range::from_str);
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect::<Vec<_>>();

    let result_1 = input
        .iter()
        .filter(|(e1, e2)| e1.contains(e2) || e2.contains(e1))
        .count();

    let result_2 = input.iter().filter(|(e1, e2)| e1.overlaps(e2)).count();

    (result_1, result_2)
}

base_aoc!(2, 4);
