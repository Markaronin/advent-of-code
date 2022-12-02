project_name=$1

if [ -d ./$project_name ]; then
    echo "Project already exists"
    exit 1
fi

sed -i "`wc -l < Cargo.toml`i\\    \"$1\",\\" Cargo.toml

cargo init $1

cat >./$1/src/main.rs << END
use advent_of_code_util::*;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    (0, 0)
}

base_aoc!(0, 0);
END

echo "advent_of_code_util = {path = \"../advent_of_code_util\"}" >> $1/Cargo.toml

code ./$1/src/main.rs

input_file=./$1/input
touch $input_file
code $input_file
test_input_file=./$1/testinput
touch $test_input_file
code $test_input_file