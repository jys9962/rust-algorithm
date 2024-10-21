use std::io::{self, BufWriter, Write};

fn solution(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' => stack.push('('),
            ')' => {
                if stack.is_empty() || stack.pop() != Some('(') {
                    return false;
                }
            }
            '[' => stack.push('['),
            ']' => {
                if stack.is_empty() || stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {} // 괄호가 아닌 다른 문자 무시
        }
    }

    stack.is_empty()
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = BufWriter::new(io::stdout().lock());

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim_end();

        // 종료 조건
        if input == "." {
            break;
        }

        // solution 함수 호출 및 결과 출력
        let result = if solution(input) { "yes" } else { "no" };
        writeln!(stdout, "{}", result).unwrap();
    }

    // 버퍼된 출력 내용을 flush
    stdout.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(solution("So when I die (the [first] I will see in (heaven) is a score list)."), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(solution("[ first in ] ( first out )."), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(solution("Half Moon tonight (At least it is better than no Moon at all]."), false);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(solution("A rope may form )( a trail in a maze."), false);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(solution("Help( I[m being held prisoner in a fortune cookie factory)]."), false);
    }

    #[test]
    fn test_case_6() {
        assert_eq!(solution("([ (([( [ ] ) ( ) (( ))] )) ])."), true);
    }
}
