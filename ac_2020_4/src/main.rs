use advent_of_code_util::*;
use itertools::Itertools;
use regex::Regex;

fn does_match_year(val: &str, year_min: usize, year_max: usize) -> bool {
    let year_re = Regex::new("^[0-9]+$").unwrap();
    year_re.is_match(&val) && {
        let parsed_year = val.parse::<usize>().unwrap();
        parsed_year >= year_min && parsed_year <= year_max
    }
}
fn is_valid_height(height: &str) -> bool {
    let height_regex = Regex::new("^[0-9]+(in|cm)$").unwrap();
    height_regex.is_match(&height) && {
        let mut chars = height.chars();
        let units = [chars.next_back().unwrap(), chars.next_back().unwrap()]
            .iter()
            .rev()
            .collect::<String>();
        let amount = chars.collect::<String>().parse::<usize>().unwrap();
        (units == "cm" && amount <= 193 && amount >= 150)
            || (units == "in" && amount <= 76 && amount >= 59)
    }
}
fn is_valid_hair_color(hair_color: &str) -> bool {
    let hair_color_regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    hair_color_regex.is_match(&hair_color)
}
fn is_valid_eye_color(eye_color: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color)
}
fn is_valid_pid(pid: &str) -> bool {
    let pid_regex = Regex::new("^[0-9]{9}$").unwrap();
    pid_regex.is_match(&pid)
}

#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn from_block(block: Vec<String>) -> Self {
        let mut new_passport = Passport::default();
        split_block_on_whitespace(block)
            .iter()
            .map(|entry| entry.split(':').collect_tuple::<(&str, &str)>().unwrap())
            .for_each(|entry| match entry {
                ("byr", val) => new_passport.byr = Some(val.to_string()),
                ("iyr", val) => new_passport.iyr = Some(val.to_string()),
                ("eyr", val) => new_passport.eyr = Some(val.to_string()),
                ("hgt", val) => new_passport.hgt = Some(val.to_string()),
                ("hcl", val) => new_passport.hcl = Some(val.to_string()),
                ("ecl", val) => new_passport.ecl = Some(val.to_string()),
                ("pid", val) => new_passport.pid = Some(val.to_string()),
                ("cid", val) => new_passport.cid = Some(val.to_string()),
                _ => panic!(),
            });
        new_passport
    }

    fn has_required_fields(&self) -> bool {
        [
            &self.byr, &self.iyr, &self.eyr, &self.pid, &self.hgt, &self.hcl, &self.ecl,
        ]
        .iter()
        .all(|val| val.is_some())
    }

    fn fields_are_valid(&self) -> bool {
        [
            self.byr
                .clone()
                .map(|byr| does_match_year(&byr, 1920, 2002)),
            self.iyr
                .clone()
                .map(|iyr| does_match_year(&iyr, 2010, 2020)),
            self.eyr
                .clone()
                .map(|eyr| does_match_year(&eyr, 2020, 2030)),
            self.hgt.clone().map(|hgt| is_valid_height(&hgt)),
            self.hcl.clone().map(|hcl| is_valid_hair_color(&hcl)),
            self.ecl.clone().map(|ecl| is_valid_eye_color(&ecl)),
            self.pid.clone().map(|pid| is_valid_pid(&pid)),
        ]
        .into_iter()
        .flatten()
        .all(|item| item)
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file);
    let passports = input
        .into_iter()
        .map(|block| Passport::from_block(block))
        .collect::<Vec<Passport>>();
    (
        passports
            .iter()
            .filter(|passport| passport.has_required_fields())
            .count(),
        passports
            .iter()
            .filter(|passport| passport.has_required_fields() && passport.fields_are_valid())
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_match_year_test() {
        assert!(does_match_year("1993", 1993, 1997));
        assert!(!does_match_year("1992", 1993, 1997));
        assert!(does_match_year("1997", 1993, 1997));
        assert!(!does_match_year("1998", 1993, 1997));
    }
    #[test]
    fn is_valid_height_test() {
        assert!(is_valid_height("60in"));
        assert!(is_valid_height("190cm"));
        assert!(!is_valid_height("190in"));
        assert!(!is_valid_height("190"));
    }
    #[test]
    fn is_valid_hair_color_test() {
        assert!(is_valid_hair_color("#123abc"));
        assert!(!is_valid_hair_color("#123abz"));
        assert!(!is_valid_hair_color("123abc"));
    }
    #[test]
    fn is_valid_eye_color_test() {
        assert!(is_valid_eye_color("brn"));
        assert!(!is_valid_eye_color("wat"));
    }
    #[test]
    fn is_valid_pid_test() {
        assert!(is_valid_pid("000000001"));
        assert!(!is_valid_pid("0123456789"));
    }

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 10);
        assert_eq!(part_2_output, 6);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
