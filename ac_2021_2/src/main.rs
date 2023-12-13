use advent_of_code_util::parse::read_lines;

#[derive(Debug)]
enum CommandType {
    Up,
    Down,
    Forward,
}

impl CommandType {
    fn from_string(string: &str) -> Self {
        match string {
            "up" => CommandType::Up,
            "down" => CommandType::Down,
            "forward" => CommandType::Forward,
            _ => panic!("Unrecognized command type"),
        }
    }
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    amount: u32,
}

impl Command {
    fn from_line(line: &str) -> Self {
        let separated_line = line.split_whitespace().collect::<Vec<&str>>();
        Command {
            command_type: CommandType::from_string(separated_line[0]),
            amount: separated_line[1].parse::<u32>().unwrap(),
        }
    }
}

fn main() {
    let lines = read_lines("ac_2021_2/input");
    let commands = lines
        .iter()
        .map(|line| Command::from_line(&line))
        .collect::<Vec<Command>>();

    let mut part_1_x = 0;
    let mut part_1_depth = 0;
    for command in &commands {
        match command.command_type {
            CommandType::Up => part_1_depth -= command.amount,
            CommandType::Down => part_1_depth += command.amount,
            CommandType::Forward => part_1_x += command.amount,
        }
    }
    let part_1_score = part_1_x * part_1_depth;
    println!("Horizontal position * depth: {}", part_1_score);

    let mut part_2_x = 0;
    let mut part_2_aim = 0;
    let mut part_2_depth = 0;
    for command in &commands {
        match command.command_type {
            CommandType::Up => part_2_aim -= command.amount,
            CommandType::Down => part_2_aim += command.amount,
            CommandType::Forward => {
                part_2_x += command.amount;
                part_2_depth += part_2_aim * command.amount;
            }
        }
    }
    let part_2_score = part_2_x * part_2_depth;
    println!("Horizontal position * depth, part 2: {}", part_2_score);
}
