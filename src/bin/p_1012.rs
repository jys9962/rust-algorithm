use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let stdout = io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = first_line.trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let m = parts[0]; // 가로 길이
        let n = parts[1]; // 세로 길이
        let k = parts[2]; // 배추 위치 개수

        // 배추밭 초기화
        let mut field = vec![vec![false; m]; n];

        for _ in 0..k {
            let coords = lines.next().unwrap().unwrap();
            let xy: Vec<usize> = coords.trim().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            let x = xy[0]; // X 좌표
            let y = xy[1]; // Y 좌표

            field[y][x] = true; // 배추가 심어져 있는 위치 표시
        }

        // 솔루션을 작성하세요.
        let result = solution(&mut field);
        writeln!(buffer, "{}", result).unwrap();
    }
}

// 솔루션을 작성하세요.
fn solution(field: &mut Vec<Vec<bool>>) -> usize {
    let mut check_map: Vec<Vec<bool>> = vec![vec![false; field[0].len()]; field.len()];
    let mut result = 0;

    for y in 0..field.len() {
        for x in 0..field[y].len() {
            if !check_map[y][x] && field[y][x] {
                result += 1;
                go(y, x, field, &mut check_map);
            }
        }
    }

    result
}

fn go(y: usize, x: usize, field: &mut Vec<Vec<bool>>, checked: &mut Vec<Vec<bool>>) {
    if checked[y][x] || !field[y][x] {
        return;
    }

    checked[y][x] = true;
    if y > 0 {
        go(y - 1, x, field, checked);
    }
    if x > 0 {
        go(y, x - 1, field, checked);
    }
    if y < field.len() - 1 {
        go(y + 1, x, field, checked);
    }
    if x < field[0].len() - 1 {
        go(y, x + 1, field, checked);
    }
}

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut field = vec![
            vec![true, true, false, false, false, false, false, false, false, false],
            vec![false, true, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, true, false, false, false, false, false],
            vec![false, false, false, false, true, false, false, false, false, false],
            vec![false, false, true, true, false, false, false, true, true, true],
            vec![false, false, false, false, true, false, false, true, true, true],
        ];
        let result = solution(&mut field);
        let expected = 5; // 예제에서의 결과

        assert_eq!(result, expected);
    }
}
