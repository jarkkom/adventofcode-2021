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

fn read_input(reader: impl Read) -> Result<Vec<i64>, String> {
    let reader = BufReader::new(reader);

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => output.push(i64::from_str_radix(&x, 2).unwrap_or_default()),
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(output)
}

fn count_bits(inputs: &[i64]) -> (i64, i64) {
    let mut one_most_common = 0;
    let mut zero_most_common = 0;

    let input_length = 64 - inputs.iter().map(|x| x.leading_zeros()).min().unwrap();

    for i in 0..input_length {
        one_most_common <<= 1;
        zero_most_common <<= 1;

        let mut zeros = 0;
        let mut ones = 0;

        let mask = 1 << (input_length - 1 - i);
        for b in inputs.iter() {
            match b & mask == mask {
                true => ones += 1,
                false => zeros += 1,
            }
        }

        if ones >= zeros {
            one_most_common |= 1;
        }
        if zeros >= ones {
            zero_most_common |= 1;
        }
    }
    (one_most_common, zero_most_common)
}

fn get_oxygen_rating(inputs: &[i64]) -> i64 {
    let mut workset = inputs.to_owned();

    let input_length = 64 - inputs.iter().map(|x| x.leading_zeros()).min().unwrap();

    let mut bit_idx = 1 << (input_length - 1);
    for _ in 0..input_length {
        let (ones_common, zeros_common) = count_bits(&workset);

        let one_most_common = (ones_common & bit_idx) != 0;
        let zero_most_common = (zeros_common & bit_idx) != 0;

        workset.retain(|b| {
            let c = b & bit_idx != 0;

            one_most_common && c || !one_most_common && zero_most_common && !c
        });
        bit_idx >>= 1;

        if workset.len() == 1 {
            break;
        }
    }

    workset[0]
}

fn get_co2_scrubber_rating(inputs: &[i64]) -> i64 {
    let mut workset = inputs.to_owned();

    let input_length = 64 - inputs.iter().map(|x| x.leading_zeros()).min().unwrap();

    let mut bit_idx = 1 << (input_length - 1);
    for _ in 0..input_length {
        let (ones_common, zeros_common) = count_bits(&workset);

        let one_most_common = (ones_common & bit_idx) != 0;
        let zero_most_common = (zeros_common & bit_idx) != 0;

        workset.retain(|b| {
            let c = b & bit_idx != 0;

            one_most_common && !c || !one_most_common && zero_most_common && c
        });
        bit_idx >>= 1;

        if workset.len() == 1 {
            break;
        }
    }

    workset[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            let oxygen_rating = get_oxygen_rating(&inputs);
            let co2_scrubber_rating = get_co2_scrubber_rating(&inputs);
            println!("answer {:?}", oxygen_rating * co2_scrubber_rating);
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = String::from(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );

        let expected = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(read_input(input.as_bytes()).unwrap(), expected);
    }

    #[test]
    fn test_count_bits() {
        let input: Vec<i64> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(count_bits(&input), (0b10110, 0b01001));
    }

    #[test]
    fn test_get_oxygen_rating() {
        let input: Vec<i64> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(get_oxygen_rating(&input), 0b10111);
    }

    #[test]
    fn test_get_co2_scrubber_rating() {
        let input: Vec<i64> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(get_co2_scrubber_rating(&input), 0b01010);
    }
}
