project_name=$1

if [ -d ./$project_name ]; then
    echo "Project already exists"
    exit 1
fi

cargo init $1

cat >./$1/src/main.rs << END
use advent_of_code_util::*;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    (0, 0)
}

base_aoc!(0, 0);
END

cat >./$1/Cargo.toml << END
[package]
name = "$1"
version.workspace = true
edition.workspace = true

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
advent_of_code_util = {path = "../advent_of_code_util"}
itertools.workspace = true
END

code ./$1/src/main.rs

input_file=./$1/input
touch $input_file
code $input_file
test_input_file=./$1/testinput
touch $test_input_file
code $test_input_file