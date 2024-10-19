use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let result = solution(
        line.split_whitespace()
            .collect()
    );

    println!("{result}");
}

fn solution(list: Vec<&str>) -> i32 {
    list.iter().count() as i32
}
