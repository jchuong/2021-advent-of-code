use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let mut horizontal = 0;
    let mut depth = 0;
    for line in lines {
        let instruction: Vec<&str> = line.as_ref().unwrap().split(' ').collect();
        let direction = instruction[0];
        let distance = instruction[1].parse::<u32>().unwrap();
        match direction {
            "up" => depth = depth - distance,
            "down" => depth = depth + distance,
            "forward" => horizontal = horizontal + distance,
            _ => println!("error!")
        }
    }
    println!("{}", horizontal * depth);
}
