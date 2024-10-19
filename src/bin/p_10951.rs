use std::io;
use std::io::BufRead;

fn main() {
    let handle = io::stdin()
        .lock();


    for line in handle.lines() {
        let numbers: Vec<u8> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let result = solution(numbers[0], numbers[1]);
        println!("{result}");
    }
}

fn solution(a: u8, b: u8) -> u8 {
    a + b
}
