/// 동치 관계의 속성 판별 및 동치류 계산을 수행하는 모듈

use crate::Matrix;

/// 관계가 반사성(reflexive)을 만족하는지 판별하는 함수
/// 모든 i에 대해 R(i,i) = 1인지 확인
pub fn is_reflexive(matrix: &Matrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        if matrix[i][i] != 1 {
            return false;
        }
    }
    true
}

/// 관계가 대칭성(symmetric)을 만족하는지 판별하는 함수
/// 모든 i,j에 대해 R(i,j) = R(j,i)인지 확인
pub fn is_symmetric(matrix: &Matrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] != matrix[j][i] {
                return false;
            }
        }
    }
    true
}

/// 관계가 추이성(transitive)을 만족하는지 판별하는 함수
/// R(i,j) ∧ R(j,k) → R(i,k) 조건이 모든 i,j,k에 대해 성립하는지 확인
pub fn is_transitive(matrix: &Matrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if matrix[i][j] == 1 && matrix[j][k] == 1 && matrix[i][k] != 1 {
                    return false;
                }
            }
        }
    }
    true
}

pub fn is_equivalence_relation(matrix: &RelationMatrix) -> bool {
    is_reflexive(matrix) && is_symmetric(matrix) && is_transitive(matrix)
}

pub fn print_equivalence_result(matrix: &RelationMatrix) {
    println!("\n=== 동치 관계 판별 결과 ===");

    let reflexive = is_reflexive(matrix);
    let symmetric = is_symmetric(matrix);
    let transitive = is_transitive(matrix);
    let equivalence = is_equivalence_relation(matrix);

    println!("반사성 (Reflexive): {}", if reflexive { "✓ 만족" } else { "✗ 불만족" });
    println!("대칭성 (Symmetric): {}", if symmetric { "✓ 만족" } else { "✗ 불만족" });
    println!("추이성 (Transitive): {}", if transitive { "✓ 만족" } else { "✗ 불만족" });

    print_additional_properties(matrix);

    println!("\n{}", if equivalence {
        "이 관계는 동치 관계입니다!"
    } else {
        "이 관계는 동치 관계가 아닙니다."
    });

    // 동치 관계일 경우 동치류 출력
    if equivalence {
        super::visualize::print_equivalence_classes(matrix);
    }
}

