use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

#[derive(PartialEq, Debug)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[derive(PartialEq, Debug)]
struct Submarine {
    depth: i64,
    x: i64,
}

fn parse_command(command: &str) -> Option<Command> {
    let parts = command.split_ascii_whitespace().collect::<Vec<&str>>();

    if parts.len() != 2 {
        return None;
    }

    match parts[0] {
        "forward" => Some(Command::Forward(parts[1].parse::<i64>().unwrap())),
        "down" => Some(Command::Down(parts[1].parse::<i64>().unwrap())),
        "up" => Some(Command::Up(parts[1].parse::<i64>().unwrap())),
        _ => None,
    }
}

fn open_input(filename: &str) -> io::Result<File> {
    let path = Path::new(filename);
    File::open(path)
}

fn read_input(reader: impl Read) -> Result<Vec<Command>, String> {
    let reader = BufReader::new(reader);

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => match parse_command(&x) {
                Some(cmd) => output.push(cmd),
                None => continue,
            },
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(output)
}

impl Submarine {
    fn run_command(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(n) => self.x += n,
            Command::Up(n) => self.depth -= n,
            Command::Down(n) => self.depth += n,
        }
    }

    fn run_commands(&mut self, inputs: Vec<Command>) {
        for i in inputs {
            self.run_command(i);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let mut submarine = Submarine { x: 0, depth: 0 };

    let input_file = open_input(&filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            submarine.run_commands(inputs);
            println!("answer {:?}", submarine.x * submarine.depth);
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
            "forward 5
down 5
forward 8
up 3
down 8
forward 2
",
        );

        let expected = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(read_input(input.as_bytes()).unwrap(), expected);
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(parse_command("forward 123"), Some(Command::Forward(123)));
        assert_eq!(parse_command("up -456"), Some(Command::Up(-456)));
        assert_eq!(parse_command("down 789"), Some(Command::Down(789)));
        assert_eq!(parse_command("sdfsdf"), None);
    }

    #[test]
    fn test_run_commands() {
        let mut submarine = Submarine { x: 0, depth: 0 };
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        submarine.run_commands(commands);
        assert_eq!(submarine.x, 15);
        assert_eq!(submarine.depth, 10);
    }
}
