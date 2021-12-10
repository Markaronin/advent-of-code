use advent_of_code_util::read_lines;

fn get_program_output(input_file: &str) {
    let input = read_lines(input_file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let output = get_program_output(&file_path);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let output = get_program_output(&file_path);
}
