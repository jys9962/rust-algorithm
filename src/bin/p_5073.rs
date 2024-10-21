use std::io::{self, BufWriter, Write};

fn solution(a: i32, b: i32, c: i32) -> &'static str {
    let mut list = [a, b, c];
    list.sort();

    if list[0] == list[2] {
        return "Equilateral";
    }
    if list[2] >= list[0] + list[1] {
        return "Invalid";
    }
    if list[0] == list[1] || list[1] == list[2] {
        return "Isosceles";
    }

    "Scalene"
}


fn main() {
    let stdin = io::stdin();
    let mut stdout = BufWriter::new(io::stdout().lock());

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (a, b, c) = (nums[0], nums[1], nums[2]);

        // 종료 조건
        if a == 0 && b == 0 && c == 0 {
            break;
        }

        // solution 함수로부터 결과를 받아서 버퍼를 이용해 출력
        let result = solution(a, b, c);
        writeln!(stdout, "{}", result).unwrap();
    }

    // 버퍼된 출력 내용을 flush
    stdout.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        // 예제 테스트 케이스
        let test_cases = vec![
            (7, 7, 7, "Equilateral"),
            (6, 5, 4, "Scalene"),
            (3, 2, 5, "Invalid"),
            (6, 2, 6, "Isosceles"),
        ];

        for (a, b, c, expected) in test_cases {
            let result = solution(a, b, c);
            assert_eq!(result.trim(), expected);
        }
    }
}
