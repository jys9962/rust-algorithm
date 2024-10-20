fn solution(x: i32) -> usize {
    format!("{:02b}", x).chars().filter(|&c| c == '1').count()
}

fn main() {
    // 입출력 처리
    use std::io::{self, BufWriter, Write};
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let x: i32 = input.trim().parse().unwrap();

    let result = solution(x); // 솔루션 호출

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", result).unwrap(); // 결과 출력
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let x = 23;
        let result = solution(x);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_case_2() {
        let x = 32;
        let result = solution(x);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_3() {
        let x = 64;
        let result = solution(x);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_case_4() {
        let x = 48;
        let result = solution(x);
        assert_eq!(result, 2);
    }
}
