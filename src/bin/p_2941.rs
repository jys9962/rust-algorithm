use std::io;

fn main() {
    let mut input = String::new();

    // 입력을 받는다.
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 공백 및 개행 문자 제거
    let word = input.trim();

    // 크로아티아 알파벳 개수 확인
    let count = solution(word);

    // 결과 출력
    println!("{}", count);
}

fn solution(word: &str) -> usize {
    let mut count = 0;
    let mut i = 0;

    while i < word.len() {
        count += 1;
        if i + 2 < word.len() && &word[i..i + 3] == "dz=" {
            i += 3;
        } else if i + 1 < word.len() && &word[i..i + 2] == "c=" {
            i += 2;
        } else if i + 1 < word.len() && &word[i..i + 2] == "c-" {
            i += 2;
        } else if i + 1 < word.len() && &word[i..i + 2] == "d-" {
            i += 2;
        } else if i + 1 < word.len() && &word[i..i + 2] == "lj" {
            i += 2;
        } else if i + 1 < word.len() && &word[i..i + 2] == "nj" {
            i += 2;
        } else if i + 1 < word.len() && &word[i..i + 2] == "s=" {
            i += 2;
        } else if i + 1 < word.len() && &word[i..i + 2] == "z=" {
            i += 2;
        } else {
            i += 1;
        }
    }

    count
}
// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_croatian_alphabet() {
        assert_eq!(solution("ljes=njak"), 6);
        assert_eq!(solution("ddz=z="), 3);
        assert_eq!(solution("nljj"), 3);
        assert_eq!(solution("c=c="), 2);
        assert_eq!(solution("dz=ak"), 3);
    }
}
