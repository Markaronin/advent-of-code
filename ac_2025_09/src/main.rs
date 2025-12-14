use advent_of_code_util::{Coordinate, base_aoc, parse::read_parsed_lines};

struct Polygon {
    points: Vec<Coordinate>,
}
impl Polygon {
    pub fn is_point_inside_or_on_edge(&self, other: &Coordinate) -> bool {
        unimplemented!()
    }
}

fn get_all_points_along_edge_of_square(
    first_corner: &Coordinate,
    third_corner: &Coordinate,
) -> Vec<Coordinate> {
    let second_corner = Coordinate {
        x: first_corner.x,
        y: third_corner.y,
    };
    let fourth_corner = Coordinate {
        x: third_corner.x,
        y: first_corner.y,
    };

    let mut result = Vec::new();

    result.append(&mut first_corner.get_points_between_vertices(&second_corner));
    result.append(&mut second_corner.get_points_between_vertices(third_corner));
    result.append(&mut third_corner.get_points_between_vertices(&fourth_corner));
    result.append(&mut fourth_corner.get_points_between_vertices(first_corner));
    result
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<Coordinate> = read_parsed_lines(input_file);

    let poly = Polygon {
        points: input.clone(),
    };

    // Create polygon
    // For each possible square, first check if it's bigger than the biggest possible max, then check if each point on the square is within the polygon (use the even-odd rule?)

    let mut part_1 = 0;
    let mut part_2 = 0;

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let square_size =
                (input[i].x.abs_diff(input[j].x) + 1) * (input[i].y.abs_diff(input[j].y) + 1);
            part_1 = part_1.max(square_size);

            if square_size > part_2
                && get_all_points_along_edge_of_square(&input[i], &input[j])
                    .into_iter()
                    .all(|p| poly.is_point_inside_or_on_edge(&p))
            {
                part_2 = square_size;
            }
        }
    }

    (part_1, part_2)
}

base_aoc!(50, 0);
