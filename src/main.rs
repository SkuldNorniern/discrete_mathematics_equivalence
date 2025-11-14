/// 애플리케이션의 메인 진입점 및 사용자 인터페이스 모듈

use std::io::{self, Write};

use discrete_mathematics_equivalence::{
    equivalence::{
        print_equivalence_result
    },
    visualize::{
        print_text_visualization, print_matrix,
        analyze_individual_properties, demonstrate_equivalence_classes, analyze_relationship_properties
    },
    Matrix,
};
use discrete_mathematics_equivalence::closure::perform_closure_analysis;

/// 프로그램의 메인 함수
/// 관계 행렬 입력, 동치 관계 판별, 시각화, 폐포 분석을 순차적으로 수행
fn main() {
    println!("=== 관계행렬 입력 및 동치 관계 판별 ===\n");

    // 동적 크기의 관계행렬 입력받기 (2×2 ~ 10×10)
    match read_relation_matrix() {
        Ok(matrix) => {
            print_matrix(&matrix, "입력된 관계행렬");

            // 동치 관계 판별
            print_equivalence_result(&matrix);

            // 개별 속성 상세 분석
            analyze_individual_properties(&matrix);

            // 동치류 상세 분석 및 데모
            demonstrate_equivalence_classes(&matrix);

            // 관계 속성 종합 분석
            analyze_relationship_properties(&matrix);

            // 텍스트 기반 시각화 (인접 리스트, 연결 요소)
            print_text_visualization(&matrix);

            // 폐포 분석 (동치 관계가 아닐 경우)
            perform_closure_analysis(&matrix);
        }
        Err(e) => {
            eprintln!("오류: {}", e);
            return;
        }
    }
}

/// 동적 크기의 관계행렬을 사용자로부터 입력받는 함수
/// 행렬 크기는 2×2부터 10×10까지 지원하며, 유효성 검사를 수행
/// 잘못된 입력시 재입력을 요청하며, 성공시 Matrix를 반환
fn read_relation_matrix() -> Result<Matrix, String> {
    // 행렬 크기 입력받기
    let size = loop {
        println!("관계행렬의 크기를 입력하세요 (2-10 사이의 정수): ");
        print!("크기: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(size) => {
                if size >= 2 && size <= 10 {
                    break size;
                } else {
                    println!("오류: 크기는 2에서 10 사이여야 합니다. 다시 입력하세요.");
                    continue;
                }
            }
            Err(_) => {
                println!("오류: 유효한 정수를 입력하세요. 다시 입력하세요.");
                continue;
            }
        }
    };

    println!("\n{}×{} 관계행렬을 행 단위로 입력하세요.", size, size);
    println!("각 행은 공백으로 구분된 {}개의 숫자(0 또는 1)를 입력하세요.\n", size);

    let mut matrix = Vec::new();

    for i in 0..size {
        loop {
            print!("행 {}: ", i + 1);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let row: Result<Vec<u8>, _> = input
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u8>())
                .collect();
            match row {
                Ok(row) => {
                    if row.len() != size {
                        println!("오류: 각 행은 정확히 {}개의 요소를 가져야 합니다. 다시 입력하세요.", size);
                        continue;
                    }
                    if row.iter().any(|&x| x != 0 && x != 1) {
                        println!("오류: 관계행렬의 요소는 0 또는 1이어야 합니다. 다시 입력하세요.");
                        continue;
                    }
                    matrix.push(row);
                    break;
                }
                Err(_) => {
                    println!("오류: 숫자가 아닌 값이 입력되었습니다. 다시 입력하세요.");
                    continue;
                }
            }
        }
    }
    Ok(matrix)
}

