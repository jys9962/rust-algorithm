use std::io::{self, BufRead};

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


fn solution(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(solution(1, 2), 3);
        assert_eq!(solution(0, 0), 0);
        assert_eq!(solution(-1, 1), 0);
        assert_eq!(solution(100, 200), 300);
    }
}
