use advent_of_code_util::{base_aoc, parse::read_blocks, Coordinate, Direction};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Block,
    Wall,
}

fn display(input: &[Vec<Space>], player_pos: Coordinate) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let coord = Coordinate { x, y };
            print!(
                "{}",
                if player_pos == coord {
                    '@'
                } else {
                    match input[y][x] {
                        Space::Empty => '.',
                        Space::Block => 'O',
                        Space::Wall => '#',
                    }
                }
            );
        }
        println!();
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: (Vec<String>, Vec<String>) =
        read_blocks(input_file).into_iter().collect_tuple().unwrap();
    let (mut player_pos, mut grid) = {
        let mut player_pos = None;
        let grid = input
            .0
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Space::Empty,
                        '#' => Space::Wall,
                        'O' => Space::Block,
                        '@' => {
                            player_pos = Some(Coordinate { x, y });
                            Space::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec();

        (player_pos.unwrap(), grid)
    };
    let moves = input
        .1
        .into_iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    for m in moves {
        let moving_into = player_pos
            .space_in_direction(m, usize::MAX, usize::MAX)
            .unwrap();
        match grid[moving_into.y][moving_into.x] {
            Space::Empty => player_pos = moving_into,
            Space::Block => {
                let mut closest_empty_space = moving_into
                    .space_in_direction(m, usize::MAX, usize::MAX)
                    .unwrap();
                while grid[closest_empty_space.y][closest_empty_space.x] == Space::Block {
                    closest_empty_space = closest_empty_space
                        .space_in_direction(m, usize::MAX, usize::MAX)
                        .unwrap();
                }
                match grid[closest_empty_space.y][closest_empty_space.x] {
                    Space::Empty => {
                        grid[closest_empty_space.y][closest_empty_space.x] = Space::Block;
                        grid[moving_into.y][moving_into.x] = Space::Empty;
                        player_pos = moving_into;
                    }
                    Space::Wall => {}
                    Space::Block => unreachable!(),
                }
            }
            Space::Wall => {}
        }
        display(&grid, player_pos);
    }

    let answer_1 = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, c)| match c {
                    Space::Empty => 0,
                    Space::Block => (y * 100) + x,
                    Space::Wall => 0,
                })
                .sum::<usize>()
        })
        .sum();

    (answer_1, 0)
}

base_aoc!(10092, 9021);
