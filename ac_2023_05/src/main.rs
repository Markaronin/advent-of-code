use std::ops::Range;

use advent_of_code_util::*;
use itertools::Itertools;

#[derive(Debug)]
struct MapRange {
    pub from_start: usize,
    pub to_start: usize,
    pub len: usize,
}
impl MapRange {
    pub fn from_line(line: &str) -> Self {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap());
        Self {
            to_start: nums.next().unwrap(),
            from_start: nums.next().unwrap(),
            len: nums.next().unwrap(),
        }
    }
    pub fn map(&self, from: usize) -> Option<usize> {
        if (self.from_start..=self.from_start + self.len).contains(&from) {
            Some(self.to_start + (from - self.from_start))
        } else {
            None
        }
    }
}

/**
 * Returns a sorted list of non-overlapping ranges
 */
fn transform_ranges(
    transformation_ranges: &Vec<MapRange>,
    mut ranges: Vec<Range<usize>>,
) -> Vec<Range<usize>> {
    // first, order the transformation ranges
    // panic if there are any overlaps
    // keep going while there are ranges left to transform
    // if the range's lower bound is higher than the tranformation range's upper bound, go to the next t range and continue
    // There are 3 cases:
    // (we should be able to do this while looking at one trange and one range at a time)
    // Chip off pieces of ranges as we map them through tranges
    // - If trange start is higher than range end, move range over, continue to next range
    // - If range start is higher than trange end, continue to next trange
    // - If trange completely covers range, transform entire range, continue to next range
    // - If range completely covers trange, move low end of range over unchanged,

    // - Trange start <= range start: chop off portion of range in trange transformed, next trange
    // - Trange start > range start: chop off range start -> trange start untransformed

    let mut transformed_ranges = vec![];

    let trange_index = 0;
    while ranges.len() > 0 {
        if trange_index == transformation_ranges.len() {
            // There are no more tranges at this point
            transformed_ranges.append(&mut ranges);
        } else {
            let trange = &transformation_ranges[trange_index];
            let range = &mut ranges[0];
            if trange.from_start <= range.start {
                if trange.from_start + trange.len > range.end {
                    // send full range transformed, next range
                    transformed_ranges.push(range.clone());
                    ranges.remove(0);
                    todo!()
                } else {
                    // chop off portion of range in trange transformed, next trange
                    todo!()
                }
            } else {
                if range.end <= trange.from_start {
                    // Move range untransformed
                    transformed_ranges.push(range.clone());
                    ranges.remove(0);
                } else {
                    // Chop off range start until trange start untransformed
                    transformed_ranges.push(range.start..trange.from_start);
                    range.start = trange.from_start;
                }
            }
        }
    }

    transformed_ranges.sort_by_key(|r: &Range<usize>| r.start);

    let mut new_transformed_ranges = vec![];

    for range in transformed_ranges {
        if new_transformed_ranges.len() == 0 {
            new_transformed_ranges.push(range);
        } else if new_transformed_ranges.last().unwrap().end >= range.start {
            let old_last = new_transformed_ranges.pop().unwrap();
            new_transformed_ranges
                .push(old_last.start.min(range.start)..old_last.end.max(range.end))
        } else {
            new_transformed_ranges.push(range);
        }
    }

    new_transformed_ranges
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn no_transformation_ranges() {
        let transformation_ranges = vec![];
        let ranges = vec![0..10];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges.clone()),
            ranges
        )
    }

    #[test]
    fn outside_range() {
        let transformation_ranges = vec![MapRange::from_line("20 21 1")];
        let ranges = vec![0..10];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges.clone()),
            ranges
        )
    }

    #[test]
    fn trange_covers_bottom_half() {
        let transformation_ranges = vec![MapRange::from_line("0 20 5")];
        let ranges = vec![0..10];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges),
            vec![5..10, 20..24]
        )
    }

    #[test]
    fn trange_covers_top_half() {
        let transformation_ranges = vec![MapRange::from_line("5 20 5")];
        let ranges = vec![0..10];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges),
            vec![0..5, 25..30]
        )
    }

    #[test]
    fn trange_covers_both_ends() {
        let transformation_ranges =
            vec![MapRange::from_line("0 20 2"), MapRange::from_line("8 30 5")];
        let ranges = vec![0..10];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges),
            vec![2..8, 20..22, 38..40]
        )
    }

    #[test]
    fn trange_between_two_ranges() {
        let transformation_ranges = vec![MapRange::from_line("5 20 5")];
        let ranges = vec![0..6, 9..15];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges),
            vec![0..5, 10..15, 25..26, 29..30]
        )
    }

    #[test]
    fn range_covers_trange() {
        let transformation_ranges = vec![MapRange::from_line("5 20 3")];
        let ranges = vec![0..10];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges),
            vec![0..5, 9..10, 25..28]
        )
    }

    #[test]
    fn overlapping_output_ranges() {
        let transformation_ranges = vec![MapRange::from_line("0 20 5")];
        let ranges = vec![0..10, 22..30];

        assert_eq!(
            transform_ranges(&transformation_ranges, ranges),
            vec![5..10, 20..30]
        )
    }
}

fn get_min_seed_location(seeds: &Vec<usize>, maps: &Vec<Vec<MapRange>>) -> usize {
    seeds
        .iter()
        .map(|seed| {
            let mut new_seed = *seed;
            for map in maps {
                new_seed = map
                    .iter()
                    .find_map(|map_range| map_range.map(new_seed))
                    .unwrap_or(new_seed);
            }
            if new_seed % 10000 == 0 {
                println!("{new_seed}");
            }
            new_seed
        })
        .min()
        .unwrap()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_blocks(input_file).into_iter();

    let seeds = input.next().unwrap()[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let maps = input
        .map(|raw| {
            raw.into_iter()
                .skip(1)
                .map(|line| MapRange::from_line(&line))
                .sorted_by_key(|m| m.from_start)
                .collect_vec()
        })
        .collect_vec();

    let result_1 = get_min_seed_location(&seeds, &maps);

    let mut seeds_2 = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut pair| {
            let start = *pair.next().unwrap();
            let amt = *pair.next().unwrap();
            start..start + amt
        })
        .collect_vec();

    for map in maps {
        seeds_2 = transform_ranges(&map, seeds_2);
    }

    // TODO: Sort maps and seed ranges

    let result_2 = seeds_2[0].next().unwrap();

    (result_1, result_2)
}

base_aoc!(35, 46);
