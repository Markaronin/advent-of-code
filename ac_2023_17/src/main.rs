use std::collections::{BTreeMap, BinaryHeap};

use advent_of_code_util::{base_aoc, parse::read_grid_of_digits, Coordinate};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Direction {
    dx: isize,
    dy: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    heat_so_far: usize,
    position: Coordinate,
    direction: Direction,
}
impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_so_far.cmp(&self.heat_so_far)
    }
}
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_shortest_path(grid: &[Vec<usize>], min_step: usize, max_step: usize) -> usize {
    let mut best_distance_grid: BTreeMap<(Coordinate, Direction), usize> = BTreeMap::new();

    let mut heap: BinaryHeap<QueueItem> = BinaryHeap::new();
    heap.push(QueueItem {
        heat_so_far: 0,
        position: Coordinate { x: 0, y: 0 },
        direction: Direction { dx: 0, dy: 0 },
    });

    let goal = Coordinate {
        x: grid[0].len() - 1,
        y: grid.len() - 1,
    };

    while let Some(QueueItem {
        heat_so_far,
        position,
        direction,
    }) = heap.pop()
    {
        if position == goal {
            return heat_so_far;
        }
        if best_distance_grid
            .get(&(position, direction))
            .is_some_and(|&c| heat_so_far > c)
        {
            continue;
        }
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (direction.dy, direction.dx) == (dy, dx)
                || (direction.dy, direction.dx) == (-dy, -dx)
            {
                continue;
            }
            let new_direction = Direction { dx, dy };
            let mut next_cost = heat_so_far;
            for dist in 1..=max_step {
                let new_position = Coordinate {
                    y: (position.y as isize + dy * dist as isize) as usize,
                    x: (position.x as isize + dx * dist as isize) as usize,
                };
                if !new_position.is_within_bounds(0, grid[0].len() - 1, 0, grid.len() - 1) {
                    continue;
                }
                next_cost += grid[new_position.y][new_position.x];
                if dist < min_step {
                    continue;
                }
                let key = (new_position, new_direction);
                if next_cost < *best_distance_grid.get(&key).unwrap_or(&usize::MAX) {
                    best_distance_grid.insert(key, next_cost);
                    heap.push(QueueItem {
                        heat_so_far: next_cost,
                        position: new_position,
                        direction: new_direction,
                    });
                }
            }
        }
    }

    unreachable!("Couldn't find a shortest path")
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_grid_of_digits(input_file);

    let result_1 = get_shortest_path(&input, 1, 3);

    let result_2 = get_shortest_path(&input, 4, 10);

    (result_1, result_2)
}

base_aoc!(102, 94);
