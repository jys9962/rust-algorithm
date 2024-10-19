use std::collections::HashMap;
use std::io;

fn main() {
    // 빈 문자열로 초기화된 String 타입의 배열
    let mut subjects: [String; 20] = Default::default();

    // 예시: 각 요소에 값 할당
    for i in 0..20 {
        subjects[i] = format!("Subject {}", i + 1);
    }

    // 20줄 입력 받기
    for i in 0..20 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        subjects[i] = input.trim().to_string();
    }

    // solution 함수 호출하여 전공평점 계산
    let gpa = solution(&subjects);

    // 결과 출력
    println!("{:.6}", gpa);
}

// solution 함수 구현
fn solution(subjects: &[String; 20]) -> f64 {
    let mut total_credits = 0.0;
    let mut weighted_score = 0.0;

    // 등급에 따른 평점 매핑
    let grade_points: HashMap<&str, f64> = [
        ("A+", 4.5),
        ("A0", 4.0),
        ("B+", 3.5),
        ("B0", 3.0),
        ("C+", 2.5),
        ("C0", 2.0),
        ("D+", 1.5),
        ("D0", 1.0),
        ("F", 0.0),
        ("P", 0.0),
    ]
        .iter()
        .cloned()
        .collect();

    // 각 과목에 대해 학점과 등급을 파싱
    for subject in subjects {
        let parts: Vec<&str> = subject.split_whitespace().collect();
        let credit: f64 = parts[1].parse().expect("Failed to parse credit");
        let grade = parts[2];

        if grade != "P" {
            total_credits += credit; // P 등급이 아닌 경우에만 학점 합산
            weighted_score += credit * grade_points[grade]; // 평점 계산
        }
    }

    // 전공평점 계산
    if total_credits == 0.0 {
        0.0 // 학점이 0일 경우 (이론적으로 불가능함)
    } else {
        weighted_score / total_credits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        assert!((solution(&[
            String::from("ObjectOrientedProgramming1 3.0 A+"),
            String::from("IntroductiontoComputerEngineering 3.0 A+"),
            String::from("ObjectOrientedProgramming2 3.0 A0"),
            String::from("CreativeComputerEngineeringDesign 3.0 A+"),
            String::from("AssemblyLanguage 3.0 A+"),
            String::from("InternetProgramming 3.0 B0"),
            String::from("ApplicationProgramminginJava 3.0 A0"),
            String::from("SystemProgramming 3.0 B0"),
            String::from("OperatingSystem 3.0 B0"),
            String::from("WirelessCommunicationsandNetworking 3.0 C+"),
            String::from("LogicCircuits 3.0 B0"),
            String::from("DataStructure 4.0 A+"),
            String::from("MicroprocessorApplication 3.0 B+"),
            String::from("EmbeddedSoftware 3.0 C0"),
            String::from("ComputerSecurity 3.0 D+"),
            String::from("Database 3.0 C+"),
            String::from("Algorithm 3.0 B0"),
            String::from("CapstoneDesigninCSE 3.0 B+"),
            String::from("CompilerDesign 3.0 D0"),
            String::from("ProblemSolving 4.0 P"),
        ]) - 3.284483).abs() < 1e-6);

        assert!((solution(&[
            String::from("BruteForce 3.0 F"),
            String::from("Greedy 1.0 F"),
            String::from("DivideandConquer 2.0 F"),
            String::from("DynamicProgramming 3.0 F"),
            String::from("DepthFirstSearch 4.0 F"),
            String::from("BreadthFirstSearch 3.0 F"),
            String::from("ShortestPath 4.0 F"),
            String::from("DisjointSet 2.0 F"),
            String::from("MinimumSpanningTree 2.0 F"),
            String::from("TopologicalSorting 1.0 F"),
            String::from("LeastCommonAncestor 2.0 F"),
            String::from("SegmentTree 4.0 F"),
            String::from("EulerTourTechnique 3.0 F"),
            String::from("StronglyConnectedComponent 2.0 F"),
            String::from("BipartiteMatching 2.0 F"),
            String::from("MaximumFlowProblem 3.0 F"),
            String::from("SuffixArray 1.0 F"),
            String::from("HeavyLightDecomposition 4.0 F"),
            String::from("CentroidDecomposition 3.0 F"),
            String::from("SplayTree 1.0 F"),
        ]) - 0.0).abs() < 1e-6);
    }
}
