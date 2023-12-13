use advent_of_code_util::parse::read_lines;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    fn get_surrounding_coordinates(&self, max_width: usize, max_height: usize) -> Vec<Coordinate> {
        let mut surrounding_coordinates = vec![];
        if self.x > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < max_width - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < max_height - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
        }
        surrounding_coordinates
    }
}

struct HeightMap {
    data: Vec<Vec<usize>>,
}
impl HeightMap {
    fn at(&self, coord: &Coordinate) -> usize {
        self.data[coord.x][coord.y]
    }

    fn from_lines(lines: Vec<String>) -> Self {
        let mut new_height_map = HeightMap { data: vec![] };
        lines.iter().for_each(|line| {
            new_height_map.data.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect(),
            )
        });
        new_height_map
    }

    fn is_low_point(&self, coord: &Coordinate) -> bool {
        let surrounding_coordinates =
            coord.get_surrounding_coordinates(self.data.len(), self.data[0].len());
        let mut is_low_point = true;
        for surrounding_coord in surrounding_coordinates {
            if self.at(&surrounding_coord) <= self.at(coord) {
                is_low_point = false;
            }
        }
        is_low_point
    }

    fn get_basin_size(&self, low_point: &Coordinate) -> usize {
        let mut basin_so_far: Vec<Coordinate> = vec![];
        let mut basin_queue = vec![low_point.clone()];
        while let Some(queue_coord) = basin_queue.pop() {
            let surrounding_coords =
                queue_coord.get_surrounding_coordinates(self.data.len(), self.data[0].len());
            let mut surrounding_coords_not_in_basin = surrounding_coords
                .iter()
                .filter(|coord| !basin_so_far.contains(coord))
                .filter(|coord| !basin_queue.contains(coord))
                .cloned()
                .collect::<Vec<Coordinate>>();
            if self.at(&queue_coord) != 9 {
                basin_so_far.push(queue_coord);
                basin_queue.append(&mut surrounding_coords_not_in_basin);
            }
        }
        basin_so_far.len()
    }

    fn find_low_points(&self) -> Vec<Coordinate> {
        let mut low_points = vec![];
        for (x, col) in self.data.iter().enumerate() {
            for (y, _) in col.iter().enumerate() {
                let coord = Coordinate { x, y };
                if self.is_low_point(&coord) {
                    low_points.push(coord);
                }
            }
        }
        low_points
    }

    fn get_risk_level(&self, coord: &Coordinate) -> usize {
        self.at(coord) + 1
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);
    let height_map = HeightMap::from_lines(input);
    let low_points = height_map.find_low_points();
    let risk_levels_sum = low_points
        .iter()
        .map(|coord| height_map.get_risk_level(coord))
        .sum();
    let mut all_basins = low_points
        .iter()
        .map(|coord| height_map.get_basin_size(coord))
        .collect::<Vec<usize>>();
    all_basins.sort();
    let basin_size_product = all_basins.iter().rev().take(3).product();
    (risk_levels_sum, basin_size_product)
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (risk_levels_sum, basin_size_product) = get_program_output(&file_path);
    println!("Sum of risk levels: {}", risk_levels_sum);
    println!("Basin size product: {}", basin_size_product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (risk_levels_sum, basin_size_product) = get_program_output(&file_path);
        assert_eq!(risk_levels_sum, 15);
        assert_eq!(basin_size_product, 1134);
    }
}
