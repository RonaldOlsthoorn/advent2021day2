use std::io::{BufReader, BufRead};
use std::fs::File;

struct Submarine {
    depth: u32,
    aim: u32,
    long: u32
}

impl Submarine {

    fn step(&mut self, command: &Command) {

        match command  {
            Command::Forward(steps) => {self.long += steps; self.depth += self.aim * steps},
            Command::Up(steps) => self.aim -= steps,
            Command::Down(steps) => self.aim += steps
        }
    }
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32)
}

impl From<String> for Command {
    fn from(line: String) -> Self {
        let splits: Vec<&str> = line.split_whitespace().collect();

        if splits[0] == "forward" {
            return Command::Forward(splits[1].parse::<u32>().unwrap());
        } else if splits[0] == "up" {
            return Command::Up(splits[1].parse::<u32>().unwrap());
        } else {
            return Command::Down(splits[1].parse::<u32>().unwrap());
        }
    }
}

fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut dasboot = Submarine{depth: 0, aim: 0, long: 0};

    for line in reader.lines().map(|l| l.unwrap()) {
        dasboot.step(&Command::from(line));
    }

    println!("final depth: {} long {} answer {}", dasboot.depth, dasboot.long, dasboot.depth * dasboot.long);
}
