use advent_of_code_util::parse::read_lines;
use std::collections::HashMap;

fn sort_characters_in_string(str: &str) -> String {
    let mut chars: Vec<char> = str.chars().collect();
    chars.sort_unstable();
    String::from_iter(chars)
}

#[derive(Debug)]
struct RawDisplay {
    digits: [String; 10],
    output: [String; 4],
}

impl RawDisplay {
    fn from_line(line: &String) -> Self {
        let first_split = line
            .split('|')
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        let mut new_raw_display = RawDisplay {
            digits: Default::default(),
            output: Default::default(),
        };

        let digits_half = first_split[0]
            .split_whitespace()
            .map(|item| sort_characters_in_string(item))
            .collect::<Vec<String>>();
        let output_half = first_split[1]
            .split_whitespace()
            .map(|item| sort_characters_in_string(item))
            .collect::<Vec<String>>();
        for i in 0..=9 {
            new_raw_display.digits[i] = digits_half.get(i).unwrap().clone();
        }
        for i in 0..=3 {
            new_raw_display.output[i] = output_half.get(i).unwrap().clone();
        }

        new_raw_display
    }

    fn character_map(&self) -> HashMap<String, u32> {
        let one = self
            .digits
            .iter()
            .find(|digit| digit.len() == 2)
            .unwrap()
            .clone();
        let four = self
            .digits
            .iter()
            .find(|digit| digit.len() == 4)
            .unwrap()
            .clone();
        let seven = self
            .digits
            .iter()
            .find(|digit| digit.len() == 3)
            .unwrap()
            .clone();
        let eight = self
            .digits
            .iter()
            .find(|digit| digit.len() == 7)
            .unwrap()
            .clone();
        // let a = seven.chars().find(|char| !one.contains(char.clone())).unwrap();
        let five_letter_digits = self
            .digits
            .iter()
            .filter(|digit| digit.len() == 5)
            .map(|digit| digit.clone())
            .collect::<Vec<String>>();
        let six_letter_digits = self
            .digits
            .iter()
            .filter(|digit| digit.len() == 6)
            .map(|digit| digit.clone())
            .collect::<Vec<String>>();
        let c = one
            .chars()
            .find(|char| {
                six_letter_digits
                    .iter()
                    .filter(|digit| digit.contains(char.clone()))
                    .count()
                    == 2
            })
            .unwrap();
        let six = six_letter_digits
            .iter()
            .find(|digit| !digit.contains(c.clone()))
            .unwrap()
            .clone();
        let f = one.chars().find(|char| char != &c).unwrap();
        let d = four
            .chars()
            .filter(|char| !one.contains(char.clone()))
            .find(|char| {
                six_letter_digits
                    .iter()
                    .filter(|digit| !digit.contains(char.clone()))
                    .count()
                    == 1
            })
            .unwrap();
        let zero = six_letter_digits
            .iter()
            .find(|digit| !digit.contains(d.clone()))
            .unwrap()
            .clone();
        let nine = six_letter_digits
            .iter()
            .find(|digit| ![six.clone(), zero.clone()].contains(digit))
            .unwrap()
            .clone();
        let b = four.chars().find(|char| ![c, d, f].contains(char)).unwrap();
        let e = eight
            .chars()
            .find(|char| !nine.contains(char.clone()))
            .unwrap();
        let five = five_letter_digits
            .iter()
            .find(|digit| !digit.contains(c) && !digit.contains(e))
            .unwrap()
            .clone();
        let three = five_letter_digits
            .iter()
            .find(|digit| !digit.contains(b) && !digit.contains(e))
            .unwrap()
            .clone();
        let two = five_letter_digits
            .iter()
            .find(|digit| !digit.contains(b) && !digit.contains(f))
            .unwrap()
            .clone();

        HashMap::from([
            (zero, 0),
            (one, 1),
            (two, 2),
            (three, 3),
            (four, 4),
            (five, 5),
            (six, 6),
            (seven, 7),
            (eight, 8),
            (nine, 9),
        ])
    }

    fn output_number(&self) -> u32 {
        let character_map = self.character_map();
        (character_map.get(&self.output[0]).unwrap() * 1000)
            + (character_map.get(&self.output[1]).unwrap() * 100)
            + (character_map.get(&self.output[2]).unwrap() * 10)
            + character_map.get(&self.output[3]).unwrap()
    }

    fn num_unique_outputs(&self) -> usize {
        let unique_output_lengths = [2, 3, 4, 7];
        self.output
            .iter()
            .filter(|digit| unique_output_lengths.contains(&digit.len()))
            .count()
    }
}

fn main() {
    let lines = read_lines("ac_2021_8/input");
    let displays = lines
        .iter()
        .map(|line| RawDisplay::from_line(line))
        .collect::<Vec<RawDisplay>>();
    println!(
        "Number of unique outputs: {:?}",
        displays
            .iter()
            .map(|display| display.num_unique_outputs())
            .sum::<usize>()
    );
    let sum_outputs: u32 = displays.iter().map(|display| display.output_number()).sum();
    println!("Sum of outputs: {}", sum_outputs);
}
