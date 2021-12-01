use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_input(filename: &str) -> Result<Vec<i64>, String> {
    let path = Path::new(filename);
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                match x.parse::<i64>() {
                    Ok(num) => output.push(num),
                    Err(err) => return Err(format!("invalid number {:?}, {:?}", x, err)),
                }
                output.push(x.parse().unwrap());
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(output)
}

fn count_increments(inputs: &[i64]) -> usize {
    return inputs.windows(2).filter(|w| w[1] > w[0]).count();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    match read_input(&filename) {
        Ok(inputs) => println!("increment count is {}", count_increments(&inputs)),
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increments() {
        let test_inputs: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increments(&test_inputs), 7);
    }
}
