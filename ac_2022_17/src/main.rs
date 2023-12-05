use advent_of_code_util::*;
use itertools::Itertools;

const NEW_ROCKS: [[[SpaceType; 4]; 4]; 5] = [
    [
        [SpaceType::Empty; 4],
        [SpaceType::Empty; 4],
        [SpaceType::Empty; 4],
        [SpaceType::FallingRock; 4],
    ],
    [
        [
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::Empty,
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::FallingRock,
            SpaceType::FallingRock,
            SpaceType::Empty,
        ],
        [
            SpaceType::Empty,
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
    ],
    [
        [
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::FallingRock,
            SpaceType::Empty,
        ],
        [
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::FallingRock,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::FallingRock,
            SpaceType::FallingRock,
            SpaceType::Empty,
        ],
    ],
    [
        [
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
    ],
    [
        [
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
        [
            SpaceType::FallingRock,
            SpaceType::FallingRock,
            SpaceType::Empty,
            SpaceType::Empty,
        ],
    ],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpaceType {
    RestingRock,
    FallingRock,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JetDirection {
    Left,
    Right,
}
impl JetDirection {
    fn from_string(s: &str) -> Vec<Self> {
        s.chars()
            .map(|c| match c {
                '<' => JetDirection::Left,
                '>' => JetDirection::Right,
                _ => panic!("Unrecognized jet direction"),
            })
            .collect_vec()
    }
}

#[derive(Debug)]
struct Rocks {
    air_jet_index: usize,
    new_rock_index: usize,
    jet_pattern: Vec<JetDirection>,
    bottom_edge_of_falling_rock: Option<usize>,
    area: Vec<[SpaceType; 7]>,
}
impl Rocks {
    pub fn new(jet_pattern: Vec<JetDirection>) -> Self {
        Self {
            air_jet_index: 0,
            new_rock_index: 0,
            jet_pattern,
            bottom_edge_of_falling_rock: None,
            area: vec![[SpaceType::RestingRock; 7]],
        }
    }

    fn remove_empty_rows(&mut self) {
        while *self.area.last().unwrap() == [SpaceType::Empty; 7] {
            self.area.pop();
        }
    }

    fn will_collide_falling(&self) -> bool {
        unimplemented!()
    }
    fn fall(&mut self) {
        unimplemented!()
    }

    fn will_collide_air_jet(&self) -> bool {
        unimplemented!()
    }
    fn air_jet(&mut self) {
        for x in match self.jet_pattern[self.air_jet_index] {
            JetDirection::Left => 1..=6,
            JetDirection::Right => 5..=0,
        } {
            for y in self.bottom_edge_of_falling_rock.unwrap()
                ..=self.bottom_edge_of_falling_rock.unwrap() + 4
            {
                if self.area[y][x] == SpaceType::FallingRock {
                    match self.jet_pattern[self.air_jet_index] {
                        JetDirection::Left => self.area[y][x - 1] = SpaceType::FallingRock,
                        JetDirection::Right => self.area[y][x + 1] = SpaceType::FallingRock,
                    }
                    self.area[y][x] = SpaceType::Empty;
                }
            }
        }

        self.air_jet_index = (self.air_jet_index + 1) % self.jet_pattern.len();
    }

    fn land_rock(&mut self) {
        for y in self.bottom_edge_of_falling_rock.unwrap()
            ..=self.bottom_edge_of_falling_rock.unwrap() + 4
        {
            for s in &mut self.area[y] {
                if *s == SpaceType::FallingRock {
                    *s = SpaceType::RestingRock;
                }
            }
        }
        self.bottom_edge_of_falling_rock = None;
    }

    fn create_new_rock(&mut self) {
        self.remove_empty_rows();

        // Add 3 empty rows, then add 4 more for shape
        for _ in 0..7 {
            self.area.push([SpaceType::Empty; 7]);
        }

        for y in 0..4 {
            for x in 0..4 {
                let area_y = self.area.len() - 1 - y;

                self.area[area_y][x + 2] = NEW_ROCKS[self.new_rock_index][y][x];
            }
        }

        self.new_rock_index = (self.new_rock_index + 1) % NEW_ROCKS.len();
        self.bottom_edge_of_falling_rock = Some(self.area.len() - 4);
    }

    pub fn drop_new_rock(&mut self) {
        self.create_new_rock();

        while self.bottom_edge_of_falling_rock.is_some() {
            if !self.will_collide_air_jet() {
                self.air_jet();
            }
            if self.will_collide_falling() {
                self.land_rock();
            } else {
                self.fall();
            }
        }
    }

    pub fn height(&mut self) -> usize {
        self.remove_empty_rows();
        self.area.len() - 1
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    // Probably part 2 is spinnning rocks
    // Check if stopping by looking for RestingRock below FallingRock, but only before downward movement
    // Check if cancelling jet by checking collision with wall + RestingRock beside FallingRock in correct direction

    let mut rocks = Rocks::new(JetDirection::from_string(&input[0]));

    for _ in 0..2022 {
        rocks.drop_new_rock();
    }

    (rocks.height(), 0)
}

base_aoc_ignore_tests!(3068, 0);
