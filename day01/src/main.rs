use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let window_size = args[1].to_string().parse::<usize>().unwrap();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let mut counter = 0;
    let mut sum = 0;

    // parse input into vec before using, and set up our window
    let numbers = lines.map(|line| line.unwrap().to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    for n in 0..window_size {
        sum = sum + numbers[n];
    }
    let mut first_number = numbers[0];

    // slide the window by checking new size and removing old number
    for n in window_size..numbers.len() {
        let new_sum = sum - first_number + numbers[n];
        if new_sum > sum {
            counter = counter + 1;
        }
        first_number = numbers[n - window_size + 1];
        sum = new_sum;
    }
    println!("{}", counter);
}
