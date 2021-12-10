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

fn parse_line(line: &str) -> Entry {
    let parts: Vec<&str> = line.split('|').collect();

    Entry {
        patterns: parts[0].split_whitespace().map(|s| s.to_owned()).collect(),
        outputs: parts[1].split_whitespace().map(|s| s.to_owned()).collect(),
    }
}

#[derive(PartialEq, Debug)]
struct Entry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

fn find_digits(outputs: &[String], segments: usize) -> usize {
    outputs.iter().filter(|s| s.len() == segments).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            let entries: Vec<Entry> = inputs.iter().map(|l| parse_line(l)).collect();

            let ones: usize = entries.iter().map(|e| find_digits(&e.outputs, 2)).sum();
            let fours: usize = entries.iter().map(|e| find_digits(&e.outputs, 4)).sum();
            let sevens: usize = entries.iter().map(|e| find_digits(&e.outputs, 3)).sum();
            let eights: usize = entries.iter().map(|e| find_digits(&e.outputs, 7)).sum();

            println!("answer {:?}", ones + fours + sevens + eights);
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");

        let expected_patterns = vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")];
        let expected_outputs = vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")];

        let expected = Entry {
            patterns: expected_patterns,
            outputs: expected_outputs,
        };

        assert_eq!(parse_line(&input), expected);
    }

    #[test]
    fn test_find_digits() {
        let outputs = vec![String::from("a"), String::from("aaa"), String::from("bbb"), String::from("cccc"), String::from("dddd"), String::from("eeeee")];

        assert_eq!(find_digits(&outputs, 1), 1);
        assert_eq!(find_digits(&outputs, 2), 0);
        assert_eq!(find_digits(&outputs, 3), 2);
        assert_eq!(find_digits(&outputs, 4), 2);
        assert_eq!(find_digits(&outputs, 5), 1);
    }

}
