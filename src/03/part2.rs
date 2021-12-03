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

fn count_bits(inputs: &[String]) -> (i64, i64) {
    let mut one_most_common = 0;
    let mut zero_most_common = 0;
    for i in 0..inputs[0].len() {
        one_most_common <<= 1;
        zero_most_common <<= 1;

        let mut zeros = 0;
        let mut ones = 0;
        for b in inputs.iter() {
            match b.as_bytes()[i] as char {
                '1' => ones += 1,
                '0' => zeros += 1,
                _ => continue,
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

fn get_oxygen_rating(inputs: &[String]) -> String {
    let mut workset = inputs.to_owned();

    let mut bit_idx = 1 << (workset[0].len() - 1);
    for i in 0..workset[0].len() {
        let (ones_common, zeros_common) = count_bits(&workset);

        let one_most_common = (ones_common & bit_idx) != 0;
        let zero_most_common = (zeros_common & bit_idx) != 0;

        workset.retain(|b| {
            let bb = b.as_bytes();
            let c = bb[i] as char;

            one_most_common && c == '1' || !one_most_common && zero_most_common && c == '0'
        });
        bit_idx >>= 1;

        if workset.len() == 1 {
            break;
        }
    }

    workset[0].clone()
}

fn get_co2_scrubber_rating(inputs: &[String]) -> String {
    let mut workset = inputs.to_owned();

    let mut bit_idx = 1 << (workset[0].len() - 1);
    for i in 0..workset[0].len() {
        let (ones_common, zeros_common) = count_bits(&workset);

        let one_most_common = (ones_common & bit_idx) != 0;
        let zero_most_common = (zeros_common & bit_idx) != 0;

        workset.retain(|b| {
            let bb = b.as_bytes();
            let c = bb[i] as char;

            one_most_common && c == '0' || !one_most_common && zero_most_common && c == '1'
        });
        bit_idx >>= 1;

        if workset.len() == 1 {
            break;
        }
    }

    workset[0].clone()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            let oxygen_rating = i64::from_str_radix(&get_oxygen_rating(&inputs), 2).unwrap();
            let co2_scrubber_rating =
                i64::from_str_radix(&get_co2_scrubber_rating(&inputs), 2).unwrap();
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
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(read_input(input.as_bytes()).unwrap(), expected);
    }

    #[test]
    fn test_count_bits() {
        let input: Vec<String> = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(count_bits(&input), (0b10110, 0b01001));
    }

    #[test]
    fn test_get_oxygen_rating() {
        let input: Vec<String> = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(get_oxygen_rating(&input), "10111");
    }

    #[test]
    fn test_get_co2_scrubber_rating() {
        let input: Vec<String> = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(get_co2_scrubber_rating(&input), "01010");
    }
}
