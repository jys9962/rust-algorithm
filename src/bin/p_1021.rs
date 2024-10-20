use std::cmp::min;
use std::collections::VecDeque;

fn solution(n: usize, m: usize, items: Vec<usize>) -> usize {
    let mut queue: VecDeque<usize> = (1..=n).collect();
    let mut result: usize = 0;

    for &item in items.iter() {
        let mut count = 0;
        loop {
            let current = queue.pop_front().unwrap();
            if item == current {
                break;
            }
            queue.push_back(current);
            count += 1;
        }

        result += min(count, queue.len() + 1 - count)
    }

    result
}

fn main() {
    // 입출력 처리
    use std::io::{self, BufWriter, Write};
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let positions: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let result = solution(n, m, positions); // 솔루션 호출

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{}", result).unwrap(); // 결과 출력
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 10;
        let m = 3;
        let positions = vec![1, 2, 3];
        let result = solution(n, m, positions);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_case_2() {
        let n = 10;
        let m = 3;
        let positions = vec![2, 9, 5];
        let result = solution(n, m, positions);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_case_3() {
        let n = 32;
        let m = 6;
        let positions = vec![27, 16, 30, 11, 6, 23];
        let result = solution(n, m, positions);
        assert_eq!(result, 59);
    }

    #[test]
    fn test_case_4() {
        let n = 10;
        let m = 10;
        let positions = vec![1, 6, 3, 2, 7, 9, 8, 4, 10, 5];
        let result = solution(n, m, positions);
        assert_eq!(result, 14);
    }
}
