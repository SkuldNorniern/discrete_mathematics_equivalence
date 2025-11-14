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

/// 관계가 동치 관계인지 판별하는 함수
/// 반사성, 대칭성, 추이성이 모두 만족하는지 확인
pub fn is_equivalence_relation(matrix: &Matrix) -> bool {
    is_reflexive(matrix) && is_symmetric(matrix) && is_transitive(matrix)
}

/// 관계가 반대칭성(antisymmetric)을 만족하는지 판별하는 함수
/// 모든 i ≠ j에 대해, R(i,j)와 R(j,i)가 동시에 성립하지 않아야 함
pub fn is_antisymmetric(matrix: &Matrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            if i != j && matrix[i][j] == 1 && matrix[j][i] == 1 {
                return false;
            }
        }
    }
    true
}

/// 관계가 비반사성(irreflexive)을 만족하는지 판별하는 함수
/// 모든 i에 대해, R(i,i)가 성립하지 않아야 함
pub fn is_irreflexive(matrix: &Matrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        if matrix[i][i] != 0 {
            return false;
        }
    }
    true
}

/// 관계가 연결성(connectedness)을 만족하는지 판별하는 함수
/// 모든 서로 다른 i, j에 대해 R(i,j) 또는 R(j,i) 중 하나는 참이어야 함
pub fn is_connected_relation(matrix: &Matrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            if i != j && !(matrix[i][j] == 1 || matrix[j][i] == 1) {
                return false;
            }
        }
    }
    true
}

/// 반대칭성, 비반사성, 연결성의 판별 결과를 출력하는 함수
/// 각 속성의 만족 여부를 한글로 표시
pub fn print_additional_properties(matrix: &Matrix) {
    println!("\n=== 추가 속성 판별 ===");
    println!(
        "반대칭성 (Antisymmetric): {}",
        if is_antisymmetric(matrix) {
            "만족"
        } else {
            "불만족"
        }
    );
    println!(
        "비반사성 (Irreflexive): {}",
        if is_irreflexive(matrix) {
            "만족"
        } else {
            "불만족"
        }
    );
}

/// 동치 관계 판별 결과를 출력하는 함수
/// 반사성, 대칭성, 추이성의 만족 여부를 표시하고 동치 관계일 경우 동치류도 출력
pub fn print_equivalence_result(matrix: &Matrix) {
    println!("\n=== 동치 관계 판별 결과 ===");

    let reflexive = is_reflexive(matrix);
    let symmetric = is_symmetric(matrix);
    let transitive = is_transitive(matrix);
    let equivalence = is_equivalence_relation(matrix);

    println!(
        "반사성 (Reflexive): {}",
        if reflexive {
            "✓ 만족"
        } else {
            "✗ 불만족"
        }
    );
    println!(
        "대칭성 (Symmetric): {}",
        if symmetric {
            "✓ 만족"
        } else {
            "✗ 불만족"
        }
    );
    println!(
        "추이성 (Transitive): {}",
        if transitive {
            "✓ 만족"
        } else {
            "✗ 불만족"
        }
    );

    print_additional_properties(matrix);

    println!(
        "\n{}",
        if equivalence {
            "이 관계는 동치 관계입니다!"
        } else {
            "이 관계는 동치 관계가 아닙니다."
        }
    );

    // 동치 관계일 경우 동치류 출력
    if equivalence {
        super::visualize::print_equivalence_classes(matrix);
    }
}
