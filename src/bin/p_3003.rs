use std::io;

fn main() {
    let mut input = String::new();

    // 입력을 받는다.
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 공백으로 구분된 입력을 벡터로 변환
    let pieces : Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    // 피스 개수를 확인하고 결과 출력
    let result = solution(&pieces);
    let result = result.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" ");
    println!("{result}");
}

// 로직을 작성하세요.
fn solution(pieces: &[i32]) -> Vec<i32> {
    // 정해진 체스 피스의 개수
    let correct_counts = [1, 1, 2, 2, 2, 8];

    // 현재 피스의 개수와 올바른 개수의 차이를 계산
    pieces.iter()
        .enumerate()
        .map(|(i, &count)| correct_counts[i] - count)
        .collect()
}

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chess_pieces() {
        assert_eq!(solution(&[0, 1, 2, 2, 2, 7]), vec![1, 0, 0, 0, 0, 1]);
        assert_eq!(solution(&[2, 1, 2, 1, 2, 1]), vec![-1, 0, 0, 1, 0, 7]);
        assert_eq!(solution(&[1, 1, 2, 2, 2, 8]), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(solution(&[0, 0, 0, 0, 0, 0]), vec![1, 1, 2, 2, 2, 8]);
    }
}
