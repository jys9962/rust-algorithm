use std::io::BufRead;

fn main() {
    let handle = std::io::stdin().lock();
    let mut lines = handle.lines();
    lines.next();
    let numbers = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();

    let result = solution(numbers);

    println!("{} {}", result.0, result.1);
}

fn solution(list: Vec<i32>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for num in list.iter() {
        if *num < min {
            min = *num;
        }
        if *num > max {
            max = *num
        }
    }

    (min, max)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(solution(vec![1, 5, 2, 3, 1]), (1, 5));
    }
}
