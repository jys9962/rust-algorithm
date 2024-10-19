use std::collections::HashMap;
use std::io;

fn main() {
    // 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // 공백 제거 및 대문자로 변환
    let word = input.trim();

    // 로직 구현을 위한 solution 함수 호출
    let result = solution(&word);

    // 결과 출력
    println!("{}", result);
}

// solution 함수 구현
fn solution(word: &str) -> char {
    let mut count_map = HashMap::new();

    for c in word.to_lowercase().chars() {
        *count_map.entry(c).or_insert(0) += 1;
    }

    let mut max_char = ' ';
    let mut max_count = 0;
    let mut has_duplicate = false;

    for (c, count) in count_map {
        if count > max_count {
            max_char = c;
            max_count = count;
            has_duplicate = false;
        }
        else if count == max_count {
            has_duplicate = true;
        }
    }

    if has_duplicate {
        '?'
    }
    else {
        max_char.to_uppercase().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        assert_eq!(solution("Mississipi"), '?');
        assert_eq!(solution("zZa"), 'Z');
        assert_eq!(solution("z"), 'Z');
        assert_eq!(solution("baaa"), 'A');
    }
}
