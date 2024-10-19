use std::io;

fn main() {
    let mut input = String::new();

    // 입력을 받는다.
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 공백 및 개행 문자 제거
    let word = input.trim();

    // 팰린드롬 여부 확인
    if solution(word) {
        println!("1");
    } else {
        println!("0");
    }
}

fn solution(word: &str) -> bool {
    let chars: Vec<_> = word.chars().collect();
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - i - 1] {
            return false;
        }
    }

    true
}

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(solution("level"), true);
        assert_eq!(solution("noon"), true);
        assert_eq!(solution("baekjoon"), false);
        assert_eq!(solution("online"), false);
        assert_eq!(solution("judge"), false);
        assert_eq!(solution("a"), true);
        assert_eq!(solution("racecar"), true);
    }
}
