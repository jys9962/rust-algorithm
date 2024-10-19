use std::collections::HashSet;
use std::io::{self, BufRead};

// 메인 함수: 입력 처리 및 결과 출력
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 첫 번째 줄에서 단어의 개수 N을 입력받음
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // N개의 단어를 벡터로 입력받음
    let words: Vec<String> = lines.take(n)
        .map(|line| line.unwrap().trim().to_string())
        .collect();

    // solution 함수 호출하여 결과 출력
    let result = solution(&words);
    println!("{}", result);
}

// 로직을 작성하세요.
fn solution(words: &[String]) -> usize {
    let mut count = 0;
    for word in words.iter() {
        if is_group_word(word) {
            count += 1
        }
    }

    count
}

fn is_group_word(word: &str) -> bool {
    let mut last_char = ' ';
    let mut dic: HashSet<char> = HashSet::new();

    for char in word.chars() {
        if char != last_char && dic.contains(&char) {
            return false;
        }

        last_char = char;
        dic.insert(char);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec!["happy".to_string(), "new".to_string(), "year".to_string()];
        assert_eq!(solution(&words), 3);
    }

    #[test]
    fn test_example_2() {
        let words = vec!["aba".to_string(), "abab".to_string(), "abcabc".to_string(), "a".to_string()];
        assert_eq!(solution(&words), 1);
    }

    #[test]
    fn test_example_3() {
        let words = vec!["ab".to_string(), "aa".to_string(), "aca".to_string(), "ba".to_string(), "bb".to_string()];
        assert_eq!(solution(&words), 4);
    }

    #[test]
    fn test_example_4() {
        let words = vec!["yzyzy".to_string(), "zyzyz".to_string()];
        assert_eq!(solution(&words), 0);
    }

    #[test]
    fn test_example_5() {
        let words = vec!["z".to_string()];
        assert_eq!(solution(&words), 1);
    }

    #[test]
    fn test_example_6() {
        let words = vec![
            "aaa".to_string(), "aaazbz".to_string(), "babb".to_string(),
            "aazz".to_string(), "azbz".to_string(), "aabbaa".to_string(),
            "abacc".to_string(), "aba".to_string(), "zzaz".to_string()
        ];
        assert_eq!(solution(&words), 2);
    }
}
