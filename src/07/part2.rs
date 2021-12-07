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

fn parse_positions(line: String) -> Vec<i64> {
    let nums: Vec<i64> = line.split(',').map(|n| n.parse().unwrap()).collect();

    nums
}

fn fib(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum
}

fn find_least_moves(positions: &[i64]) -> i64 {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut least_fuel = i64::MAX;

    for ip in *min..max + 1 {
        let req_fuel = positions
            .iter()
            .fold(0, |acc, p| acc + fib(i64::abs(p - ip)));
        if req_fuel < least_fuel {
            least_fuel = req_fuel;
        }
        //println!("pos {:?}, fuel {:?}", ip, req_fuel);
    }

    least_fuel
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(mut inputs) => {
            let positions = parse_positions(inputs.remove(0));

            println!("answer {:?}", find_least_moves(&positions));
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");

        let expected = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(parse_positions(input), expected);
    }

    #[test]
    fn test_run_gens() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(find_least_moves(&positions), 168);
    }
}
