use advent_of_code_util::parse::read_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    fn from_str(str: &str) -> Self {
        let separated = str.split(",").collect::<Vec<&str>>();
        Coordinate {
            x: separated[0].parse::<usize>().unwrap(),
            y: separated[1].parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    from: Coordinate,
    to: Coordinate,
}
impl Line {
    fn from_str(str: &str) -> Self {
        let separated = str.split(" -> ").collect::<Vec<&str>>();
        Line {
            from: Coordinate::from_str(separated[0]),
            to: Coordinate::from_str(separated[1]),
        }
    }
    fn is_diagonal(&self) -> bool {
        self.from.x != self.to.x && self.from.y != self.to.y
    }
    fn into_iter(self) -> LineIntoIterator {
        LineIntoIterator {
            current_coordinate: self.from.clone(),
            line: self,
        }
    }
}
struct LineIntoIterator {
    line: Line,
    current_coordinate: Coordinate,
}
impl Iterator for LineIntoIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let incr_x: i64 = if self.line.from.x > self.line.to.x {
            -1
        } else if self.line.from.x < self.line.to.x {
            1
        } else {
            0
        };
        let incr_y: i64 = if self.line.from.y > self.line.to.y {
            -1
        } else if self.line.from.y < self.line.to.y {
            1
        } else {
            0
        };
        let prev_coordinate = Coordinate {
            x: (self.current_coordinate.x as i64 - incr_x) as usize,
            y: (self.current_coordinate.y as i64 - incr_y) as usize,
        };
        let next_coordinate = Coordinate {
            x: (self.current_coordinate.x as i64 + incr_x) as usize,
            y: (self.current_coordinate.y as i64 + incr_y) as usize,
        };
        let return_coordinate = if prev_coordinate == self.line.to {
            None
        } else {
            Some(self.current_coordinate.clone())
        };

        self.current_coordinate = next_coordinate;
        return_coordinate // TODO
    }
}

#[derive(Debug)]
struct Diagram {
    data: Vec<Vec<usize>>,
}
impl Diagram {
    fn with_size(width: usize, height: usize) -> Self {
        Diagram {
            data: vec![vec![0; height]; width],
        }
    }
    fn num_overlaps(&self) -> usize {
        self.data
            .iter()
            .flat_map(|column| column.iter().filter(|space| **space > 1))
            .count()
    }
    fn add_line(&mut self, line: &Line) {
        line.clone()
            .into_iter()
            .for_each(|coord| self.data[coord.x][coord.y] += 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod line {
        use super::*;

        #[test]
        fn is_diagonal() {
            let line = Line {
                from: Coordinate { x: 0, y: 0 },
                to: Coordinate { x: 0, y: 1 },
            };

            assert_eq!(line.is_diagonal(), false);

            let line = Line {
                from: Coordinate { x: 0, y: 0 },
                to: Coordinate { x: 1, y: 1 },
            };

            assert_eq!(line.is_diagonal(), true);

            let line = Line {
                from: Coordinate { x: 0, y: 0 },
                to: Coordinate { x: 1, y: 0 },
            };

            assert_eq!(line.is_diagonal(), false);
        }
    }
}

fn main() {
    let lines = read_lines("ac_2021_5/input")
        .iter()
        .map(|line| Line::from_str(line))
        .collect::<Vec<Line>>();
    let board_width = lines
        .iter()
        .map(|line| std::cmp::max(line.to.x, line.from.x))
        .max()
        .unwrap()
        + 1;
    let board_height = lines
        .iter()
        .map(|line| std::cmp::max(line.to.y, line.from.y))
        .max()
        .unwrap()
        + 1;
    let mut board = Diagram::with_size(board_width, board_height);

    for line in lines.iter().filter(|line| !line.is_diagonal()) {
        board.add_line(line);
    }

    println!("Overlaps without diagonals: {:?}", board.num_overlaps());

    let mut part_2_board = Diagram::with_size(board_width, board_height);
    for line in lines {
        part_2_board.add_line(&line);
    }
    println!("Overlaps with diagonals: {:?}", part_2_board.num_overlaps());
}
