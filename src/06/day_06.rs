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

fn parse_generations(line: String) -> Vec<i64> {
    let nums: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();

    return nums.iter().fold(vec![0; 9], |mut acc, &x| {
        acc[x] += 1;
        acc
    });
}

fn run_generation(mut gens: Vec<i64>) -> Vec<i64> {
    gens.rotate_left(1);
    gens[6] += gens[8];
    gens
}

fn sum_generation(gens: &[i64]) -> i64 {
    gens.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(mut inputs) => {
            let mut gens = parse_generations(inputs.remove(0));

            for _ in 0..80 {
                gens = run_generation(gens);
            }

            println!("answer part 1 {:?}", sum_generation(&gens));

            for _ in 80..256 {
                gens = run_generation(gens);
            }

            println!("answer part 2 {:?}", sum_generation(&gens));
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let input = String::from("3,4,3,1,2");

        let expected = vec![0, 1, 1, 2, 1, 0, 0, 0, 0];

        assert_eq!(parse_generations(input), expected);
    }

    #[test]
    fn test_run_gens() {
        let input = String::from("3,4,3,1,2");

        let mut gens = parse_generations(input);

        for _ in 0..18 {
            gens = run_generation(gens);
        }

        assert_eq!(sum_generation(&gens), 26);

        for _ in 18..80 {
            gens = run_generation(gens);
        }

        assert_eq!(sum_generation(&gens), 5934);
    }
}
