fn solution(n: usize, k: usize, array: &mut [i32]) -> i32 {
    let (_, kth, _) = array.select_nth_unstable(k - 1);
    *kth
}

fn main() {
    // 입출력 처리
    use std::io::{self, BufWriter, Write};
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut array: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = solution(n, k, &mut array);  // 솔루션 호출

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", result).unwrap();  // 결과 출력
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 5;
        let k = 2;
        let mut array = vec![4, 1, 2, 3, 5];
        let result = solution(n, k, &mut array);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_case_2() {
        let n = 3;
        let k = 1;
        let mut array = vec![-3, -1, 0];
        let result = solution(n, k, &mut array);
        assert_eq!(result, -3);
    }

    #[test]
    fn test_case_3() {
        let n = 6;
        let k = 3;
        let mut array = vec![100, 50, 200, -100, 0, 75];
        let result = solution(n, k, &mut array);
        assert_eq!(result, 50);
    }
}
