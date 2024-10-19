use std::io;
use std::io::BufRead;

fn main() {
    // 표준 입력을 받기 위한 reader 생성
    let mut lines = io::stdin()
        .lock()
        .lines();

    // 한 줄 읽기
    let input = lines.next().unwrap().unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = solution(numbers[0], numbers[1]);
    println!("{result}");
}

fn solution(a: i32, b: i32) -> &'static str {
    if a > b {
         ">"
     } else if a < b {
         "<"
     } else {
         "=="
     }
}
