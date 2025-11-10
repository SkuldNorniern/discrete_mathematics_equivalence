pub type RelationMatrix = Vec<Vec<i32>>;

pub fn is_reflexive(matrix: &RelationMatrix) -> bool {
    let n = matrix.len();
    for i in 0..n {
        if matrix[i][i] != 1 {
            return false;
        }
    }
    true
}

pub fn is_symmetric(matrix: &RelationMatrix) -> bool {
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

pub fn is_transitive(matrix: &RelationMatrix) -> bool {
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

    println!("\n{}", if equivalence {
        "이 관계는 동치 관계입니다!"
    } else {
        "이 관계는 동치 관계가 아닙니다."
    });
}
