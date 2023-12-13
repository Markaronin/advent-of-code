use std::cmp::Ordering;
use std::collections::BinaryHeap;

use advent_of_code_util::parse::read_lines;
use advent_of_code_util::Coordinate;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Coordinate,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct GridGraph {
    grid: Vec<Vec<usize>>,
}
impl GridGraph {
    fn from_lines(lines: Vec<String>) -> Self {
        GridGraph {
            grid: lines
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        }
    }

    fn beeg_grid_from_lines(lines: Vec<String>) -> Self {
        let mut grid = vec![vec![0; lines[0].len() * 5]; lines[0].len() * 5];
        for beeg_x in 0..5 {
            for beeg_y in 0..5 {
                for (leetle_x, row) in lines.iter().enumerate() {
                    for (leetle_y, col) in row.chars().enumerate() {
                        let x = (beeg_x * lines[0].len()) + leetle_x;
                        let y = (beeg_y * lines.len()) + leetle_y;
                        grid[x][y] = col.to_digit(10).unwrap() as usize + beeg_x + beeg_y;
                        while grid[x][y] > 9 {
                            grid[x][y] -= 9;
                        }
                    }
                }
            }
        }
        GridGraph { grid }
    }

    fn find_path_with_lowest_risk(&self) -> usize {
        let mut q: BinaryHeap<State> = BinaryHeap::new();
        let mut dist = vec![vec![usize::MAX; self.grid[0].len()]; self.grid.len()];
        let mut prev: Vec<Vec<Option<Coordinate>>> =
            vec![vec![None; self.grid[0].len()]; self.grid.len()];
        dist[0][0] = 0;
        q.push(State {
            cost: 0,
            position: Coordinate { x: 0, y: 0 },
        });

        let goal = Coordinate {
            x: self.grid.len() - 1,
            y: self.grid[0].len() - 1,
        };

        while let Some(State { cost, position }) = q.pop() {
            if position == goal {
                return dist[goal.x][goal.y];
            }

            if cost > dist[position.x][position.y] {
                continue;
            }

            for neighbor in position
                .get_surrounding_non_diagonal_coordinates(self.grid.len(), self.grid[0].len())
            {
                let next = State {
                    cost: cost + self.grid[neighbor.x][neighbor.y],
                    position: neighbor,
                };

                if next.cost < dist[neighbor.x][neighbor.y] {
                    q.push(next);
                    dist[neighbor.x][neighbor.y] = next.cost;
                    prev[neighbor.x][neighbor.y] = Some(position);
                }
            }
        }
        panic!("Goal not reacheable")
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let gg = GridGraph::from_lines(input.clone());
    let beeg_grid = GridGraph::beeg_grid_from_lines(input);

    (
        gg.find_path_with_lowest_risk(),
        beeg_grid.find_path_with_lowest_risk(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 40);
        assert_eq!(part_2_output, 315);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
