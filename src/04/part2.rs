use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn open_input(filename: &str) -> io::Result<File> {
    let path = Path::new(filename);
    File::open(path)
}

fn read_input(reader: impl Read) -> Result<Vec<String>, String> {
    let reader = BufReader::new(reader);

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => output.push(x),
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(output)
}

fn parse_numbers(line: String) -> Vec<i64> {
    line.split(',')
        .filter(|&x| x != "x")
        .map(|n| n.parse().unwrap())
        .collect()
}

#[derive(PartialEq, Debug)]
struct Board {
    nums: Vec<Vec<i64>>,
}

impl Board {
    fn check_win(&self, numbers: &[i64]) -> Option<i64> {
        let mut match_rows = Vec::new();
        for row in self.nums.iter() {
            let mut match_cols = Vec::new();
            for col in row.iter() {
                match_cols.push(numbers.contains(col));
            }
            match_rows.push(match_cols);
        }

        // check vertical row
        for x in 0..match_rows[0].len() {
            let mut win = true;
            for row in match_rows.iter() {
                if !row[x] {
                    win = false;
                }
            }
            if win {
                return Some(self.sum_unmarked(match_rows));
            }
        }

        // check horizontal row
        for y in 0..match_rows.len() {
            let mut win = true;
            for x in 0..match_rows[y].len() {
                if !match_rows[y][x] {
                    win = false;
                }
            }
            if win {
                return Some(self.sum_unmarked(match_rows));
            }
        }

        None
    }

    fn sum_unmarked(&self, match_rows: Vec<Vec<bool>>) -> i64 {
        let mut sum = 0;
        for (y, row) in match_rows.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if !col {
                    sum += self.nums[y][x];
                }
            }
        }
        sum
    }
}

fn parse_board(mut lines: Vec<String>) -> (Board, Vec<String>) {
    let mut res = Vec::new();

    loop {
        if lines.is_empty() {
            break;
        }
        let l = lines.remove(0);
        if l.is_empty() || l == "\n" {
            break;
        }
        res.push(
            l.split(' ')
                .filter(|&x| !x.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect(),
        );
    }
    (Board { nums: res }, lines)
}

fn play_bingo(numbers: Vec<i64>, boards: Vec<Board>) -> i64 {
    let mut last_n = 0;
    let mut last_sum = 0;
    for b in &boards {
        for n in 0..numbers.len() {
            let mut temp_n = numbers.clone();
            temp_n.truncate(n + 1);

            match b.check_win(&temp_n) {
                Some(sum) => {
                    if n > last_n {
                        last_n = n;
                        last_sum = sum * temp_n.last().unwrap();
                    }

                    break;
                }
                _ => continue,
            }
        }
    }

    last_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(mut inputs) => {
            let numbers = parse_numbers(inputs.remove(0));
            inputs.remove(0);

            let mut boards = Vec::new();
            loop {
                let board = parse_board(inputs);
                boards.push(board.0);
                inputs = board.1;
                if inputs.is_empty() {
                    break;
                }
            }

            println!("boards {:?}", boards);

            let answer = play_bingo(numbers, boards);
            println!("answer {:?}", answer);
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let input =
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1");

        let expected = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        assert_eq!(parse_numbers(input), expected);
    }

    #[test]
    fn test_parse_board() {
        let input = vec![
            String::from("22 59  7 10  6"),
            String::from("33 36 96 55 23"),
            String::from("13 85 18 29 28"),
            String::from("75 46 83 73 58"),
            String::from("34 40 87 56 98"),
            String::from(""),
        ];

        let expected = vec![
            vec![22, 59, 7, 10, 6],
            vec![33, 36, 96, 55, 23],
            vec![13, 85, 18, 29, 28],
            vec![75, 46, 83, 73, 58],
            vec![34, 40, 87, 56, 98],
        ];

        let board = parse_board(input);
        assert_eq!(board.0.nums, expected);
        assert_eq!(board.1.len(), 0);
    }

    #[test]
    fn test_board_check_win() {
        let input = vec![
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
        ];

        let parsed_board = parse_board(input);
        let board = parsed_board.0;

        let expected_loss = board.check_win(&vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21]);
        assert_eq!(expected_loss, None);

        let expected_win = board.check_win(&vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]);
        assert_eq!(expected_win, Some(188));
    }
}
