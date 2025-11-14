/// 관계 행렬의 폐포(closure) 연산을 수행하는 모듈 (반사, 대칭, 추이 폐포)

use crate::Matrix;

/// 행렬을 복사하는 헬퍼 함수 - 동치 관계 판별시 원본 행렬 보존용
fn copy_matrix(matrix: &Matrix) -> Matrix {
    matrix.clone()
}

/// 반사 폐포 계산 - 관계가 반사성을 만족하도록 모든 대각선 요소를 1로 설정
pub fn reflexive_closure(matrix: &Matrix) -> Matrix {
    let mut result = copy_matrix(matrix);
    let n = result.len();

    for i in 0..n {
        result[i][i] = 1;
    }

    result
}

/// 대칭 폐포 계산 - 관계가 대칭성을 만족하도록 R(i,j)=1이면 R(j,i)=1로 설정
pub fn symmetric_closure(matrix: &Matrix) -> Matrix {
    let mut result = copy_matrix(matrix);
    let n = result.len();

    for i in 0..n {
        for j in 0..n {
            if result[i][j] == 1 {
                result[j][i] = 1;
            }
        }
    }

    result
}

/// 추이 폐포 계산 - Floyd-Warshall 알고리즘으로 R(i,j)∧R(j,k)→R(i,k) 규칙 적용
pub fn transitive_closure(matrix: &Matrix) -> Matrix {
    let mut result = copy_matrix(matrix);
    let n = result.len();

    // Floyd-Warshall 알고리즘 적용
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if result[i][k] == 1 && result[k][j] == 1 {
                    result[i][j] = 1;
                }
            }
        }
    }

    result
}


/// 폐포 종합 분석 - 세 가지 폐포(반사/대칭/추이)를 적용하고 그 효과를 분석하여 출력
pub fn perform_closure_analysis(matrix: &Matrix) {
    println!("\n=== 폐포 분석 ===");

    let is_already_equivalence = super::equivalence::is_equivalence_relation(matrix);

    if is_already_equivalence {
        println!("이 관계는 이미 완벽한 동치 관계입니다.");
        println!("하지만 교육적 목적으로 각 폐포의 효과를 시연합니다.\n");
    }

    // 반사 폐포
    println!("--- 반사 폐포 ---");
    let reflexive_closed = reflexive_closure(matrix);
    let changed_reflexive = reflexive_closed != *matrix;

    if !super::equivalence::is_reflexive(matrix) || is_already_equivalence {
        super::visualize::print_matrix(matrix, "변환 전 행렬");
        super::visualize::print_matrix(&reflexive_closed, "반사 폐포 후 행렬");

        if changed_reflexive {
            println!("변화: 반사 폐포에 의해 행렬이 변경되었습니다.");
        } else {
            println!("변화: 이미 반사성이 만족되어 변경되지 않았습니다.");
        }

        println!("\n반사 폐포 후 동치 관계 판별:");
        super::equivalence::print_equivalence_result(&reflexive_closed);
    }

    // 대칭 폐포
    println!("\n--- 대칭 폐포 ---");
    let symmetric_closed = symmetric_closure(matrix);
    let changed_symmetric = symmetric_closed != *matrix;

    if !super::equivalence::is_symmetric(matrix) || is_already_equivalence {
        super::visualize::print_matrix(matrix, "변환 전 행렬");
        super::visualize::print_matrix(&symmetric_closed, "대칭 폐포 후 행렬");

        if changed_symmetric {
            println!("변화: 대칭 폐포에 의해 행렬이 변경되었습니다.");
        } else {
            println!("변화: 이미 대칭성이 만족되어 변경되지 않았습니다.");
        }

        println!("\n대칭 폐포 후 동치 관계 판별:");
        super::equivalence::print_equivalence_result(&symmetric_closed);
    }

    // 추이 폐포
    println!("\n--- 추이 폐포 ---");
    let transitive_closed = transitive_closure(matrix);
    let changed_transitive = transitive_closed != *matrix;

    if !super::equivalence::is_transitive(matrix) || is_already_equivalence {
        super::visualize::print_matrix(matrix, "변환 전 행렬");
        super::visualize::print_matrix(&transitive_closed, "추이 폐포 후 행렬");

        if changed_transitive {
            println!("변화: 추이 폐포에 의해 행렬이 변경되었습니다.");
        } else {
            println!("변화: 이미 추이성이 만족되어 변경되지 않았습니다.");
        }

        println!("\n추이 폐포 후 동치 관계 판별:");
        super::equivalence::print_equivalence_result(&transitive_closed);
    }

    // 모든 폐포를 한 번에 적용한 경우
    let all_closed = transitive_closure(&symmetric_closure(&reflexive_closure(matrix)));
    let changed_all = all_closed != *matrix;

    println!("\n--- 모든 폐포 적용 (반사 + 대칭 + 추이) ---");
    if !is_already_equivalence || changed_all {
        super::visualize::print_matrix(matrix, "원본 행렬");
        super::visualize::print_matrix(&all_closed, "모든 폐포 적용 후 행렬");

        if changed_all {
            println!("변화: 폐포 적용에 의해 행렬이 변경되었습니다.");
        } else {
            println!("변화: 이미 완벽한 동치 관계여서 변경되지 않았습니다.");
        }

        println!("\n모든 폐포 적용 후 동치 관계 판별:");
        super::equivalence::print_equivalence_result(&all_closed);
    } else {
        println!("원본 행렬이 이미 완벽한 동치 관계이므로 모든 폐포 적용을 건너뜁니다.");
    }
}
