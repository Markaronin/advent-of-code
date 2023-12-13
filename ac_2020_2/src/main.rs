use advent_of_code_util::parse::read_lines;
use itertools::Itertools;

struct Policy {
    range: (usize, usize),
    character: char,
}
impl Policy {
    fn from_str(string: &str) -> Self {
        let (range_string, character) = string.split(" ").collect_tuple().unwrap();
        Policy {
            range: range_string
                .split('-')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap(),
            character: character.chars().next().unwrap(),
        }
    }

    fn is_valid_1(&self, password: &str) -> bool {
        (self.range.0..=self.range.1)
            .contains(&password.chars().filter(|ch| *ch == self.character).count())
    }

    fn is_valid_2(&self, password: &str) -> bool {
        [self.range.0, self.range.1]
            .iter()
            .filter(|pos| password.chars().nth(**pos - 1).unwrap() == self.character)
            .count()
            == 1
    }
}

struct PasswordAndPolicy {
    password: String,
    policy: Policy,
}
impl PasswordAndPolicy {
    fn from_str(string: &str) -> Self {
        let separated = string.split(": ").collect::<Vec<&str>>();
        PasswordAndPolicy {
            password: separated[1].to_string(),
            policy: Policy::from_str(separated[0]),
        }
    }

    fn password_matches_policy_1(&self) -> bool {
        self.policy.is_valid_1(&self.password)
    }

    fn password_matches_policy_2(&self) -> bool {
        self.policy.is_valid_2(&self.password)
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let valid_passwords_1 = input
        .iter()
        .map(|line| PasswordAndPolicy::from_str(line))
        .filter(|pap| pap.password_matches_policy_1())
        .count();

    let valid_passwords_2 = input
        .iter()
        .map(|line| PasswordAndPolicy::from_str(line))
        .filter(|pap| pap.password_matches_policy_2())
        .count();

    (valid_passwords_1, valid_passwords_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 2);
        assert_eq!(part_2_output, 1);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
