use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let m: u16 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let n: u16 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // 솔루션을 작성하세요.
    let (sum, min_prime) = solution(m, n);

    // 결과 출력
    if sum == 0 {
        println!("-1");
    } else {
        println!("{}", sum);
        println!("{}", min_prime);
    }
}

// 솔루션을 작성하세요.
fn solution(m: u16, n: u16) -> (u32, u16) {
    let mut sum: u32 = 0;
    let mut min_prime: u16 = 0;

    let is_prime = |num: u16| -> bool {
        if num < 2 {
            return false; // 0과 1은 소수가 아님
        }
        if num == 2 {
            return true; // 2는 소수
        }
        if num % 2 == 0 {
            return false; // 짝수는 소수가 아님
        }

        // 홀수로 나누기
        for i in (3..=((num as f64).sqrt() as u16)).step_by(2) {
            if num % i == 0 {
                return false; // 나머지가 0인 경우 소수가 아님
            }
        }
        true // 소수인 경우
    };

    for num in m..n + 1 {
        if is_prime(num) {
            sum += num as u32;
            if min_prime == 0 {
                min_prime = num;
            }
        }
    }

    (sum, min_prime)
}

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(60, 100), (620, 61));
        assert_eq!(solution(64, 65), (0, 0));
        assert_eq!(solution(1, 10), (17, 2)); // 2, 3, 5, 7
    }
}
