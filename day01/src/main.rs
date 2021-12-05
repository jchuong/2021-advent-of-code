use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    let mut last_input = -1;
    let mut counter = 0;

    while let Some(line) = lines.next() {
        let curr = line.unwrap().to_string().parse::<i32>().unwrap();
        if last_input != -1 && last_input < curr {
            counter = counter + 1;
        }
        last_input = curr;
    }
    println!("{}", counter);
}
