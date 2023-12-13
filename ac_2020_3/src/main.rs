use advent_of_code_util::parse::read_lines;

struct TreeMap {
    map: Vec<String>,
}
impl TreeMap {
    fn from_lines(lines: Vec<String>) -> Self {
        TreeMap { map: lines }
    }
    fn at_position(&self, col: usize, row: usize) -> char {
        let wrapped_col = col % self.map[0].len();
        self.map[row].chars().nth(wrapped_col).unwrap()
    }
    fn map_height(&self) -> usize {
        self.map.len()
    }
}

fn hits_for_slope(map: &TreeMap, slope: (usize, usize)) -> usize {
    let mut pos = (0, 0);
    let mut hit_count = 0;
    while (pos.1) < map.map_height() {
        if map.at_position(pos.0, pos.1) == '#' {
            hit_count += 1;
        }
        pos = (pos.0 + slope.0, pos.1 + slope.1);
    }
    hit_count
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);
    let map = TreeMap::from_lines(input);
    let hit_count_1 = hits_for_slope(&map, (3, 1));
    let hit_count_product = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| hits_for_slope(&map, *slope))
        .product();

    (hit_count_1, hit_count_product)
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 7);
        assert_eq!(part_2_output, 336);
    }
}
