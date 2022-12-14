use advent_of_code_util::*;
use itertools::Itertools;

fn get_all_points(vertices: Vec<Coordinate>) -> Vec<Coordinate> {
    vertices
        .windows(2)
        .map(|points| points[0].get_points_between_vertices(&points[1]))
        .flatten()
        .unique()
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Rock,
    Sand,
    Empty,
}
impl Space {
    pub fn to_string(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Sand => 'O',
            Self::Rock => '#',
        }
    }
}

fn print_grid(
    grid: &[[Space; 1000]; 1000],
    min_x: &usize,
    max_x: &usize,
    min_y: &usize,
    max_y: &usize,
) {
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            print!("{}", grid[x][y].to_string());
        }
        print!("\n");
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let rocks = read_lines(input_file)
        .into_iter()
        .map(|line| line.split(" -> ").map(Coordinate::from_str).collect_vec())
        .map(get_all_points)
        .flatten()
        .unique()
        .collect_vec();

    let sand_spawn = Coordinate { x: 500, y: 0 };

    let min_x = rocks.iter().map(|coord| coord.x).min().unwrap();
    let max_x = rocks.iter().map(|coord| coord.x).max().unwrap();
    let min_y = 0;

    let result_1 = {
        let max_y = rocks.iter().map(|coord| coord.y).max().unwrap();

        let mut grid = [[Space::Empty; 1000]; 1000];

        for rock in rocks.iter() {
            grid[rock.x][rock.y] = Space::Rock;
        }

        print_grid(&grid, &min_x, &max_x, &min_y, &max_y);

        let mut falling_sand = sand_spawn.clone();
        while falling_sand.is_within_bounds(min_x, max_x, min_y, max_y) {
            if grid[falling_sand.x][falling_sand.y + 1] == Space::Empty {
                falling_sand.y += 1;
            } else if grid[falling_sand.x - 1][falling_sand.y + 1] == Space::Empty {
                falling_sand.y += 1;
                falling_sand.x -= 1;
            } else if grid[falling_sand.x + 1][falling_sand.y + 1] == Space::Empty {
                falling_sand.y += 1;
                falling_sand.x += 1;
            } else {
                grid[falling_sand.x][falling_sand.y] = Space::Sand;
                falling_sand = sand_spawn.clone();
            }
        }

        print_grid(&grid, &min_x, &max_x, &min_y, &max_y);

        grid.iter()
            .map(|col| col.iter().filter(|space| **space == Space::Sand).count())
            .sum()
    };

    let result_2 = {
        let max_y = rocks.iter().map(|coord| coord.y).max().unwrap() + 2;

        let mut grid = [[Space::Empty; 1000]; 1000];

        for rock in rocks.iter() {
            grid[rock.x][rock.y] = Space::Rock;
        }
        for x in 0..1000 {
            grid[x][max_y] = Space::Rock;
        }

        print_grid(&grid, &min_x, &max_x, &min_y, &max_y);

        let mut falling_sand = sand_spawn.clone();
        while grid[sand_spawn.x][sand_spawn.y] != Space::Sand {
            if grid[falling_sand.x][falling_sand.y + 1] == Space::Empty {
                falling_sand.y += 1;
            } else if grid[falling_sand.x - 1][falling_sand.y + 1] == Space::Empty {
                falling_sand.y += 1;
                falling_sand.x -= 1;
            } else if grid[falling_sand.x + 1][falling_sand.y + 1] == Space::Empty {
                falling_sand.y += 1;
                falling_sand.x += 1;
            } else {
                grid[falling_sand.x][falling_sand.y] = Space::Sand;
                falling_sand = sand_spawn.clone();
            }
        }

        print_grid(&grid, &min_x, &max_x, &min_y, &max_y);

        grid.iter()
            .map(|col| col.iter().filter(|space| **space == Space::Sand).count())
            .sum()
    };

    (result_1, result_2)
}

base_aoc!(24, 93);
