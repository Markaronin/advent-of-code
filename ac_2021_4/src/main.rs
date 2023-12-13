use advent_of_code_util::parse::read_lines;

#[derive(Debug)]
struct BingoBoard {
    board_values: [[u32; 5]; 5],
    board_done: [[bool; 5]; 5],
}

impl BingoBoard {
    fn draw_num(&mut self, &draw: &u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.board_values[x][y] == draw {
                    self.board_done[x][y] = true;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        for x in 0..5 {
            let mut has_bingo_row = true;
            let mut has_bingo_col = true;
            for y in 0..5 {
                if !self.board_done[x][y] {
                    has_bingo_row = false;
                }
                if !self.board_done[y][x] {
                    has_bingo_col = false;
                }
            }
            if has_bingo_col || has_bingo_row {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.board_done[x][y] {
                    sum += self.board_values[x][y];
                }
            }
        }
        sum
    }
}

fn bingo_board_from_chunk(chunk: Vec<String>) -> BingoBoard {
    let mut split_chunk = chunk[1..]
        .iter()
        .map(|line| line.split_whitespace())
        .map(|split_line| split_line.map(|num| num.parse::<u32>().unwrap()));
    let mut board_values = [[0; 5]; 5];
    for x in 0..5 {
        let mut current_row = split_chunk.next().unwrap();
        for y in 0..5 {
            let current_col = current_row.next().unwrap();
            board_values[x][y] = current_col;
        }
    }
    BingoBoard {
        board_values,
        board_done: [[false; 5]; 5],
    }
}

fn main() {
    let lines = read_lines("ac_2021_4/input");
    let draws = &lines[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut boards = lines[1..]
        .chunks(6)
        .map(|chunk| bingo_board_from_chunk(chunk.to_vec()))
        .collect::<Vec<BingoBoard>>();
    let mut winning_result: Option<u32> = None;
    let mut losing_result: Option<u32> = None;
    for draw in draws {
        let num_unfinished_boards = boards.iter().filter(|board| !board.has_bingo()).count();
        for board in &mut boards {
            let had_bingo = board.has_bingo();
            board.draw_num(draw);
            let has_bingo = board.has_bingo();
            if winning_result.is_none() && has_bingo {
                winning_result = Some(board.sum_unmarked() * draw);
            }
            if losing_result.is_none() && num_unfinished_boards == 1 && has_bingo && !had_bingo {
                losing_result = Some(board.sum_unmarked() * draw);
                println!("{:?}", board);
            }
        }
    }
    println!("Winning board score: {}", winning_result.unwrap());
    println!("Last winning board score: {}", losing_result.unwrap());
}
