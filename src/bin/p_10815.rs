use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let cards: HashSet<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    // let m: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let queries: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let results = solution(&cards, &queries);
    println!("{}", results.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn solution(cards: &HashSet<i32>, queries: &Vec<i32>) -> Vec<i32> {
    queries.iter().map(|t| {
        if cards.contains(t) {
            1
        } else {
            0
        }
    })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut cards = HashSet::new();
        cards.insert(6);
        cards.insert(3);
        cards.insert(2);
        cards.insert(10);
        cards.insert(-10);

        let queries = vec![10, 9, -5, 2, 3, 4, 5, -10];
        let expected = vec![1, 0, 0, 1, 1, 0, 0, 1];

        let result = solution(&cards, &queries);
        assert_eq!(result, expected);
    }
}
