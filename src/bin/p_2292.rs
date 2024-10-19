use std::cmp::max;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let result = solution(n);
    println!("{}", result);
}

fn solution(n: i32) -> i32 {
    let mut covered = 1;
    let mut result = 1;

    while n > covered {
        result += 1;
        covered += (result * 2 - 1) * 2 + max(result - 2, 0) * 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(1), 1);
        assert_eq!(solution(2), 2);
        assert_eq!(solution(7), 2);
        assert_eq!(solution(13), 3);
        assert_eq!(solution(58), 5);
    }
}
