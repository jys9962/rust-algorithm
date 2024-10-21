use std::io::{self, BufWriter, Write};

fn solution(n: i32) -> i32 {
    let mut per = n % 5;
    while per <= n {
        if per % 3 == 0 {
            return (n - per) / 5 + (per / 3);
        }

        per += 5;
    }

    -1
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = BufWriter::new(io::stdout().lock());

    // 입력 처리
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // solution 함수 호출 및 결과 출력
    let result = solution(n);
    writeln!(stdout, "{}", result).unwrap();

    // 버퍼된 출력 내용을 flush
    stdout.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(solution(18), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(solution(4), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(solution(6), 2);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(solution(9), 3);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(solution(11), 3);
    }
}
