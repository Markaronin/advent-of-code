use advent_of_code_util::read_lines;

fn num_increases(numbers: &Vec<u32>) -> u32 {
    let mut prev = numbers[0].clone();
    let mut num_increases = 0;
    for number in numbers {
        if number > &prev {
            num_increases += 1;
        }
        prev = *number;
    }
    num_increases
}

fn main() {
    let lines = read_lines("ac_2021_1/input");
    let numbers = lines
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let num_individual_increases = num_increases(&numbers);
    println!("Number of increases: {}", num_individual_increases);
    let mut sliding_window_numbers = vec![];
    for i in 2..numbers.len() {
        sliding_window_numbers.push(numbers[i - 2] + numbers[i - 1] + numbers[i])
    }
    let sliding_scale_increases = num_increases(&sliding_window_numbers);
    println!(
        "Number of sliding-scale increases: {}",
        sliding_scale_increases
    );
}
