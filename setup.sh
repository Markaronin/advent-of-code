project_name=$1

sed -i "`wc -l < Cargo.toml`i\\    \"$1\",\\" Cargo.toml

cargo init $1

cat >./$1/src/main.rs << END
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
END

echo "advent_of_code_util = {path = \"../advent_of_code_util\"}" >> $1/Cargo.toml

code ./$1/src/main.rs

input_file=./$1/input
touch $input_file
code $input_file
test_input_file=./$1/testinput
touch $test_input_file
code $test_input_file