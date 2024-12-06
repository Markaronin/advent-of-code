use advent_of_code_util::{base_aoc, parse::read_list_of_lists};

fn is_safe(report: &[usize]) -> bool {
    assert!(report.len() >= 2);
    let increasing = report[0] <= report[1];
    for window in report.windows(2) {
        let window_increasing = window[0] <= window[1];
        if window_increasing != increasing {
            return false;
        }
        let diff = window[0].abs_diff(window[1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn is_safe_with_any_removed(report: &[usize]) -> bool {
    for i in 0..report.len() {
        let mut edited_report = report.to_vec();
        edited_report.remove(i);
        if is_safe(&edited_report) {
            return true;
        }
    }
    false
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_list_of_lists(input_file, " ", |v| v.parse::<usize>().unwrap());

    let answer_1 = input.iter().filter(|report| is_safe(report)).count();
    let answer_2 = input
        .iter()
        .filter(|report| is_safe_with_any_removed(report))
        .count();

    (answer_1, answer_2)
}

base_aoc!(2, 4);
