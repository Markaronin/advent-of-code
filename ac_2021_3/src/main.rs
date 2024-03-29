use advent_of_code_util::parse::read_lines;

fn to_binary_vector(strs: &[String]) -> Vec<usize> {
    strs.iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<usize>>()
}

fn bit_at_position(number: usize, position: usize) -> bool {
    number & 2_usize.pow(position.try_into().unwrap()) != 0
}
fn get_num_ones(numbers: &[usize], position: usize) -> usize {
    numbers
        .iter()
        .map(|number| bit_at_position(*number, position))
        .filter(|bit| *bit)
        .count()
}
fn most_common_bit(numbers: &[usize], position: usize) -> bool {
    let num_ones = get_num_ones(numbers, position) as f64;
    let half_numbers_length = numbers.len() as f64 / 2.0;
    num_ones > half_numbers_length
}
fn most_common_bit_with_tiebreaker(numbers: &[usize], position: usize, tiebreaker: bool) -> bool {
    let num_ones = get_num_ones(numbers, position) as f64;
    let half_numbers_length = numbers.len() as f64 / 2.0;
    if num_ones > half_numbers_length {
        true
    } else if num_ones < half_numbers_length {
        false
    } else {
        tiebreaker
    }
}

fn main() {
    let lines = read_lines("ac_2021_3/input");
    let binary_input = to_binary_vector(&lines);
    let gamma_rate = usize::from_str_radix(
        &(0..lines[0].len())
            .rev()
            .map(|position| most_common_bit(&binary_input, position))
            .map(|most_common_bit| match most_common_bit {
                true => '1',
                false => '0',
            })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let epsilon_rate = usize::from_str_radix(
        &(0..lines[0].len())
            .rev()
            .map(|position| most_common_bit(&binary_input, position))
            .map(|most_common_bit| match most_common_bit {
                true => '0',
                false => '1',
            })
            .collect::<String>(),
        2,
    )
    .unwrap();

    println!("Power consumption: {:?}", gamma_rate * epsilon_rate);

    let oxygen_generator_rating = {
        let mut numbers = binary_input.clone();
        let mut position = lines[0].len();
        while numbers.len() != 1 {
            position -= 1;
            let mcb = most_common_bit_with_tiebreaker(&numbers, position, true);
            numbers.retain(|number| bit_at_position(*number, position) == mcb);
        }
        numbers[0]
    };
    let co2_scrubber_rating = {
        let mut numbers = binary_input.clone();
        let mut position = lines[0].len();
        while numbers.len() != 1 {
            position -= 1;
            let mcb = most_common_bit_with_tiebreaker(&numbers, position, true);
            numbers.retain(|number| bit_at_position(*number, position) != mcb);
        }
        numbers[0]
    };
    println!(
        "Life support rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn slice_to_binary_vector(strs: &[&str]) -> Vec<usize> {
        strs.iter()
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect::<Vec<usize>>()
    }

    #[test]
    fn one_bit() {
        assert!(!most_common_bit(&slice_to_binary_vector(&["0"]), 0));

        assert!(most_common_bit(&slice_to_binary_vector(&["1"]), 0));

        assert!(!most_common_bit(&slice_to_binary_vector(&["1", "0"]), 0));

        assert!(most_common_bit(
            &slice_to_binary_vector(&["0", "1", "1"]),
            0
        ));

        assert!(!most_common_bit(
            &slice_to_binary_vector(&["0", "0", "1"]),
            0
        ));
    }

    #[test]
    fn two_bits() {
        assert!(most_common_bit(
            &slice_to_binary_vector(&["00", "01", "11"]),
            0
        ));
        assert!(!most_common_bit(&slice_to_binary_vector(&["00", "11"]), 0));
        assert!(!most_common_bit(
            &slice_to_binary_vector(&["00", "01", "11"]),
            1
        ));
    }

    #[test]
    fn tiebreaker() {
        assert!(!most_common_bit_with_tiebreaker(
            &slice_to_binary_vector(&["00", "11"]),
            0,
            false
        ));
        assert!(most_common_bit_with_tiebreaker(
            &slice_to_binary_vector(&["00", "11"]),
            0,
            true
        ));
    }
}
