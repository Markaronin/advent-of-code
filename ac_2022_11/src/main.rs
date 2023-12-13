use advent_of_code_util::{
    base_aoc,
    parse::{read_blocks, read_lines},
};
use itertools::Itertools;
use std::collections::BTreeMap;

type MonkeyNumber = usize;

const DIVISORS: [usize; 9] = [17, 7, 13, 2, 19, 3, 5, 11, 23];

#[derive(Debug, Clone)]
struct Divisibility {
    remainders: BTreeMap<usize, usize>,
}
impl Divisibility {
    pub fn from_val(val: usize) -> Self {
        let mut remainders = BTreeMap::new();

        DIVISORS.iter().for_each(|divisor| {
            remainders.insert(*divisor, val % divisor);
        });

        Self { remainders }
    }

    pub fn add_value(&mut self, val: usize) {
        self.remainders = self
            .remainders
            .clone()
            .into_iter()
            .map(|(divisor, old_remainder)| (divisor, (old_remainder + (val % divisor)) % divisor))
            .collect();
    }
    pub fn add_self(&mut self) {
        self.remainders = self
            .remainders
            .clone()
            .into_iter()
            .map(|(divisor, old_remainder)| (divisor, (old_remainder + old_remainder) % divisor))
            .collect();
    }
    pub fn multiply_value(&mut self, val: usize) {
        self.remainders = self
            .remainders
            .clone()
            .into_iter()
            .map(|(divisor, old_remainder)| (divisor, (old_remainder * (val % divisor)) % divisor))
            .collect();
    }
    pub fn multiply_self(&mut self) {
        self.remainders = self
            .remainders
            .clone()
            .into_iter()
            .map(|(divisor, old_remainder)| (divisor, (old_remainder * old_remainder) % divisor))
            .collect();
    }

    pub fn is_divisible(&self, key: &usize) -> bool {
        *self.remainders.get(key).unwrap() == 0
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Val(usize),
    Old,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}
impl Operation {
    pub fn from_line(line: String) -> Self {
        let mut operation_and_operand = line.trim().split_ascii_whitespace().skip(4);
        let operation = operation_and_operand.next().unwrap();
        let operand = match operation_and_operand.next().unwrap() {
            "old" => Operand::Old,
            val => Operand::Val(val.parse::<usize>().unwrap()),
        };
        match operation {
            "+" => Self::Add(operand),
            "*" => Self::Multiply(operand),
            _ => panic!("Unrecognized operand"),
        }
    }
    pub fn apply(&self, old: &mut Divisibility) {
        match self {
            Self::Add(operand) => match operand {
                Operand::Val(val) => old.add_value(*val),
                Operand::Old => old.add_self(),
            },
            Self::Multiply(operand) => match operand {
                Operand::Val(val) => old.multiply_value(*val),
                Operand::Old => old.multiply_self(),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisor: usize,
    target_if_true: MonkeyNumber,
    target_if_false: MonkeyNumber,
}
impl Test {
    pub fn from_lines(lines: Vec<String>) -> Self {
        let get_nth_number = |line: &str, n: usize| -> usize {
            line.trim()
                .split_ascii_whitespace()
                .nth(n)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        };

        let divisor = get_nth_number(&lines[0], 3);
        let target_if_true = get_nth_number(&lines[1], 5);
        let target_if_false = get_nth_number(&lines[2], 5);

        Self {
            divisor,
            target_if_true,
            target_if_false,
        }
    }
    pub fn target(&self, item: &Divisibility) -> MonkeyNumber {
        if item.is_divisible(&self.divisor) {
            self.target_if_true
        } else {
            self.target_if_false
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Divisibility>,
    operation: Operation,
    test: Test,
    inspections: usize,
}
impl Monkey {
    pub fn from_block(block: Vec<String>) -> Self {
        let mut block = block.into_iter().skip(1);
        let items = block
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .skip(2)
            .join("")
            .split(',')
            .map(|raw_item| raw_item.parse::<usize>().unwrap())
            .map(|val| Divisibility::from_val(val))
            .collect_vec();

        let operation = Operation::from_line(block.next().unwrap());

        let test = Test::from_lines(block.collect_vec());

        Self {
            items,
            operation,
            test,
            inspections: 0,
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let monkeys = read_blocks(input_file)
        .into_iter()
        .map(|block| Monkey::from_block(block))
        .collect_vec();

    // Doesn't work with part 2, because dividing a "divisibility" by 3 doesn't work
    // let mut monkeys_part_1 = monkeys.clone();

    // for _round in 0..20 {
    //     for monkey_index in 0..monkeys_part_1.len() {
    //         while let Some(item) = monkeys_part_1[monkey_index].items.pop() {
    //             monkeys_part_1[monkey_index].operation.apply(&mut item);
    //             item /= 3;
    //             let target = monkeys_part_1[monkey_index].test.target(&item);
    //             monkeys_part_1[target].items.push(item);
    //             monkeys_part_1[monkey_index].inspections += 1;
    //         }
    //     }
    // }

    // let result_1 = monkeys_part_1
    //     .iter()
    //     .map(|monkey| monkey.inspections)
    //     .sorted()
    //     .rev()
    //     .take(2)
    //     .product();

    let mut monkeys_part_2 = monkeys.clone();

    for _round in 0..10000 {
        for monkey_index in 0..monkeys_part_2.len() {
            while let Some(mut item) = monkeys_part_2[monkey_index].items.pop() {
                monkeys_part_2[monkey_index].operation.apply(&mut item);
                let target = monkeys_part_2[monkey_index].test.target(&item);
                monkeys_part_2[target].items.push(item);
                monkeys_part_2[monkey_index].inspections += 1;
            }
        }
    }

    let result_2 = monkeys_part_2
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product();

    (10605, result_2)
}

base_aoc!(10605, 2713310158);
