use advent_of_code_util::{base_aoc, parse::read_lines};

fn is_tree_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let mut max_left = -1;
    let mut max_right = -1;
    let mut max_up = -1;
    let mut max_down = -1;

    for inner_x in 0..x {
        max_left = std::cmp::max(max_left, grid[y][inner_x] as i32);
    }
    for inner_x in x + 1..grid[0].len() {
        max_right = std::cmp::max(max_right, grid[y][inner_x] as i32);
    }
    for inner_y in 0..y {
        max_up = std::cmp::max(max_up, grid[inner_y][x] as i32);
    }
    for inner_y in y + 1..grid.len() {
        max_down = std::cmp::max(max_down, grid[inner_y][x] as i32);
    }

    let tree_height = grid[y][x] as i32;
    if tree_height > max_left
        || tree_height > max_right
        || tree_height > max_up
        || tree_height > max_down
    {
        true
    } else {
        false
    }
}

fn scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let tree_height = grid[y][x];

    let mut max_left = (0..x)
        .rev()
        .take_while(|inner_x| tree_height > grid[y][*inner_x])
        .count();
    if max_left < x {
        max_left += 1;
    }
    let mut max_right = (x + 1..grid[0].len())
        .take_while(|inner_x| tree_height > grid[y][*inner_x])
        .count();
    if max_right < grid[1].len() - (x + 1) {
        max_right += 1;
    }
    let mut max_up = (0..y)
        .rev()
        .take_while(|inner_y| tree_height > grid[*inner_y][x])
        .count();
    if max_up < y {
        max_up += 1;
    }
    let mut max_down = (y + 1..grid.len())
        .take_while(|inner_y| tree_height > grid[*inner_y][x])
        .count();
    if max_down < grid.len() - (y + 1) {
        max_down += 1;
    }

    (max_left * max_right * max_up * max_down) as u32
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let width = input[0].len();
    let height = input.len();

    let mut result_1 = 0;

    for x in 0..width {
        for y in 0..height {
            if is_tree_visible(&input, x, y) {
                result_1 += 1;
            }
        }
    }

    let mut result_2 = 0;

    for x in 0..width {
        for y in 0..height {
            result_2 = std::cmp::max(result_2, scenic_score(&input, x, y) as usize);
        }
    }

    (result_1, result_2)
}

base_aoc!(21, 8);
