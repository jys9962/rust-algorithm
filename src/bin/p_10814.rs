use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // 회원 정보를 저장할 벡터
    let mut members = Vec::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let age: u32 = parts[0].parse().unwrap();
        let name = parts[1].to_string();
        members.push((age, name));
    }

    members.sort_by(|a, b| a.0.cmp(&b.0));

    let stdout = io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());

    for (age, name) in members {
        writeln!(buffer, "{} {}", age, name).unwrap();
    }
}

// 솔루션을 작성하세요.
