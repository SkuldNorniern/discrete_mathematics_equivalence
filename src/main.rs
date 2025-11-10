use std::io::{self, Write};

type RelationMatrix = Vec<Vec<i32>>;

fn main() {
    println!("=== 관계행렬 입력 및 동치 관계 판별 ===\n");

    // 5x5 관계행렬 입력받기
    match read_relation_matrix() {
        Ok(matrix) => {
            println!("\n=== 입력된 관계행렬 ===");
            print_matrix(&matrix);
        }
        Err(e) => {
            eprintln!("오류: {}", e);
            return;
        }
    }
}

/// 5×5 관계행렬을 행 단위로 입력받는 함수
/// 각 요소는 0 또는 1이어야 함
fn read_relation_matrix() -> Result<RelationMatrix, String> {
    println!("5×5 관계행렬을 행 단위로 입력하세요.");
    println!("각 행은 공백으로 구분된 5개의 숫자(0 또는 1)를 입력하세요.\n");

    let mut matrix = Vec::new();

    for i in 0..5 {
        loop {
            print!("행 {}: ", i + 1);
            if io::stdout().flush().is_err() {
                return Err("출력 버퍼 플러시 실패".to_string());
            }

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                return Err("입력 읽기 실패".to_string());
            }

            let row: Result<Vec<i32>, _> = input
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>())
                .collect();

            match row {
                Ok(row) => {
                    if row.len() != 5 {
                        println!("오류: 각 행은 정확히 5개의 요소를 가져야 합니다. 다시 입력하세요.");
                        continue;
                    }

                    // 각 요소가 0 또는 1인지 검증
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

/// 관계행렬을 출력하는 함수
fn print_matrix(matrix: &RelationMatrix) {
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}
