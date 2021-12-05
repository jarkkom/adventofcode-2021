use regex::Regex;
use std::collections::HashMap;
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

fn parse_line(line: &str) -> Line {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let cap = re.captures(line).unwrap();

    Line {
        start: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
        end: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
    }
}

#[derive(PartialEq, Debug)]
struct Line {
    start: (i64, i64),
    end: (i64, i64),
}

fn draw_line(line: Line, mut map: HashMap<(i64, i64), i64>) -> HashMap<(i64, i64), i64> {
    if line.start.0 == line.end.0 {
        let sy = i64::min(line.start.1, line.end.1);
        let ey = i64::max(line.start.1, line.end.1);
        for y in sy..ey + 1 {
            let k = (line.start.0, y);
            map.insert(k, map.get(&k).unwrap_or(&0) + 1);
        }
    }

    if line.start.1 == line.end.1 {
        let sx = i64::min(line.start.0, line.end.0);
        let ex = i64::max(line.start.0, line.end.0);
        for x in sx..ex + 1 {
            let k = (x, line.start.1);
            map.insert(k, map.get(&k).unwrap_or(&0) + 1);
        }
    }

    map
}

fn count_overlapping_points(lines: Vec<Line>, mut map: HashMap<(i64, i64), i64>) -> usize {
    for l in lines {
        map = draw_line(l, map);
    }

    map.iter().filter(|(_, &v)| v > 1).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            let lines: Vec<Line> = inputs.iter().map(|l| parse_line(l)).collect();

            let map: HashMap<(i64, i64), i64> = HashMap::new();

            let answer = count_overlapping_points(lines, map);

            println!("answer {:?}", answer);
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = String::from("0,9 -> 5,9");

        let expected = Line {
            start: (0, 9),
            end: (5, 9),
        };

        assert_eq!(parse_line(&input), expected);
    }

    #[test]
    fn test_draw_line() {
        let l1 = Line {
            start: (0, 3),
            end: (5, 3),
        };

        let l2 = Line {
            start: (3, 0),
            end: (3, 5),
        };

        let mut actual_map: HashMap<(i64, i64), i64> = HashMap::new();

        let mut expected_map: HashMap<(i64, i64), i64> = HashMap::new();
        expected_map.insert((0, 3), 1);
        expected_map.insert((1, 3), 1);
        expected_map.insert((2, 3), 1);
        expected_map.insert((3, 3), 1);
        expected_map.insert((4, 3), 1);
        expected_map.insert((5, 3), 1);

        actual_map = draw_line(l1, actual_map);

        assert_eq!(actual_map, expected_map);

        expected_map.insert((3, 0), 1);
        expected_map.insert((3, 1), 1);
        expected_map.insert((3, 2), 1);
        expected_map.insert((3, 3), 2);
        expected_map.insert((3, 4), 1);
        expected_map.insert((3, 5), 1);

        actual_map = draw_line(l2, actual_map);
        assert_eq!(actual_map, expected_map);
    }

    #[test]
    fn test_count_overlapping_points() {
        let input = String::from(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );

        let inputs = read_input(input.as_bytes()).unwrap();

        let lines: Vec<Line> = inputs.iter().map(|l| parse_line(l)).collect();

        let map: HashMap<(i64, i64), i64> = HashMap::new();

        assert_eq!(count_overlapping_points(lines, map), 5);
    }
}
