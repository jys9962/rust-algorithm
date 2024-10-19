use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut numbers: Vec<i32> = Vec::new();

    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        numbers.push(num);
    }

    // 솔루션을 작성하세요.
    let (mean, median, mode, range) = solution(numbers);

    let stdout = io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());

    writeln!(buffer, "{}", mean).unwrap();
    writeln!(buffer, "{}", median).unwrap();
    writeln!(buffer, "{}", mode).unwrap();
    writeln!(buffer, "{}", range).unwrap();
}

// 솔루션을 작성하세요.
fn solution(mut numbers: Vec<i32>) -> (i32, i32, i32, i32) {
    numbers.sort();
    let mut count_map = HashMap::new();

    let min_number = numbers[0];
    let max_number = numbers[numbers.len() - 1];
    let sum: i32 = numbers.iter().sum();
    for number in numbers.iter() {
        *count_map.entry(*number).or_insert(0) += 1;
    }

    let mode_count = count_map.iter()
        .map(|(_, &count)| count)
        .max()
        .unwrap();

    let mut mode_list: Vec<i32> = count_map.iter()
        .filter(|(_, &count)| count == mode_count)
        .map(|(&num, _)| num)
        .collect();
    let mut mode = mode_list[0];
    if mode_list.len() > 1 {
        mode_list.sort();
        mode = mode_list[1];
    }

    let mean = (sum as f64 / numbers.len() as f64).round() as i32;
    let median = numbers[numbers.len() / 2];
    let range = max_number - min_number;
    (mean, median, mode, range)
}

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case_1() {
        let numbers = vec![1, 3, 8, -2, 2];
        let (mean, median, mode, range) = solution(numbers);
        assert_eq!(mean, 2);
        assert_eq!(median, 2);
        assert_eq!(mode, 1);
        assert_eq!(range, 10);
    }

    #[test]
    fn test_solution_case_2() {
        let numbers = vec![4000];
        let (mean, median, mode, range) = solution(numbers);
        assert_eq!(mean, 4000);
        assert_eq!(median, 4000);
        assert_eq!(mode, 4000);
        assert_eq!(range, 0);
    }

    #[test]
    fn test_solution_case_3() {
        let numbers = vec![-1, -2, -3, -1, -2];
        let (mean, median, mode, range) = solution(numbers);
        assert_eq!(mean, -2);
        assert_eq!(median, -2);
        assert_eq!(mode, -1);
        assert_eq!(range, 2);
    }

    #[test]
    fn test_solution_case_4() {
        let numbers = vec![0, 0, -1];
        let (mean, median, mode, range) = solution(numbers);
        assert_eq!(mean, 0);
        assert_eq!(median, 0);
        assert_eq!(mode, 0);
        assert_eq!(range, 1);
    }
}
