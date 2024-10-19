use std::io::{self, BufRead};

fn solution(n: usize, m: usize, matrix_a: Vec<Vec<i32>>, matrix_b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            result[i][j] = matrix_a[i][j] + matrix_b[i][j];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let nm: Vec<usize> = first_line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();
    let (n, m) = (nm[0], nm[1]);

    let mut matrix_a = Vec::new();
    let mut matrix_b = Vec::new();

    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap().unwrap()
                                .split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();
        matrix_a.push(row);
    }

    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap().unwrap()
                                .split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();
        matrix_b.push(row);
    }

    let result = solution(n, m, matrix_a, matrix_b);

    for row in result {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let n = 3;
        let m = 3;

        let matrix_a = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![0, 1, 0],
        ];

        let matrix_b = vec![
            vec![3, 3, 3],
            vec![4, 4, 4],
            vec![5, 5, 100],
        ];

        let expected = vec![
            vec![4, 4, 4],
            vec![6, 6, 6],
            vec![5, 6, 100],
        ];

        let result = solution(n, m, matrix_a, matrix_b);

        assert_eq!(result, expected);
    }
}
