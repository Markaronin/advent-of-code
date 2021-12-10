use advent_of_code_util::read_lines;

fn single_cost_part_1(old_position: i64, new_position: i64) -> u32 {
    (old_position - new_position).abs() as u32
}
fn cost_to_align_part_1(positions: &Vec<u32>, position: u32) -> u32 {
    positions
        .iter()
        .map(|p| single_cost_part_1(position as i64, p.clone() as i64))
        .sum()
}
fn single_cost_part_2(old_position: i64, new_position: i64) -> u32 {
    let n = (old_position - new_position).abs() as u32;
    n * (n + 1) / 2
}
fn cost_to_align_part_2(positions: &Vec<u32>, position: u32) -> u32 {
    positions
        .iter()
        .map(|p| single_cost_part_2(position as i64, p.clone() as i64))
        .sum()
}

fn get_minimum_fuel(input_file: &str) -> (u32, u32) {
    let input = read_lines(input_file)[0].clone();
    let crab_positions = input
        .split(',')
        .map(|position_string| position_string.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let min_crab_position = crab_positions.iter().min().unwrap().clone();
    let max_crab_position = crab_positions.iter().max().unwrap().clone();
    let mut min_fuel_part_1: u32 = u32::MAX;
    for position in min_crab_position..=max_crab_position {
        min_fuel_part_1 = std::cmp::min(
            min_fuel_part_1,
            cost_to_align_part_1(&crab_positions, position),
        )
    }
    let mut min_fuel_part_2: u32 = u32::MAX;
    for position in min_crab_position..=max_crab_position {
        min_fuel_part_2 = std::cmp::min(
            min_fuel_part_2,
            cost_to_align_part_2(&crab_positions, position),
        )
    }
    (min_fuel_part_1, min_fuel_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (min_fuel_part_1, min_fuel_part_2) = get_minimum_fuel(&file_path);
        assert_eq!(min_fuel_part_1, 37);
        assert_eq!(min_fuel_part_2, 168);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (min_fuel_part_1, min_fuel_part_2) = get_minimum_fuel(&file_path);
    println!("Minimum fuel cost, part 1: {:}", min_fuel_part_1);
    println!("Minimum fuel cost, part 2: {:}", min_fuel_part_2);
}
