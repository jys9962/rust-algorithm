use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let a = nums[0]; // 낮에 올라가는 높이
    let b = nums[1]; // 밤에 미끄러지는 높이
    let v = nums[2]; // 나무 막대의 총 높이

    // 솔루션을 작성하세요.
    let result = solution(a, b, v);
    println!("{}", result);
}

// 솔루션을 작성하세요.
fn solution(a: i32, b: i32, v: i32) -> i32 {
    if a >= v {
        return 1;
    }

    let last_day = ((v - a) as f64 / (a - b) as f64).ceil() as i32;
    last_day + 1
}

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(2, 1, 5), 4);
        assert_eq!(solution(5, 1, 6), 2);
        assert_eq!(solution(100, 99, 1000000000), 999999901);
    }
}
