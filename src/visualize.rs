/// 관계 행렬을 텍스트 기반으로 시각화하는 모듈 (그래프, 연결 요소 표시)

use crate::Matrix;
use std::collections::VecDeque;

/// 인접 리스트 형태로 그래프를 출력
pub fn print_adjacency_lists(matrix: &Matrix) {
    println!("\n=== 그래프 (인접 리스트) ===");
    let n = matrix.len();
    for i in 0..n {
        let mut neighbors = Vec::new();
        for j in 0..n {
            if matrix[i][j] == 1 {
                neighbors.push((j + 1).to_string());
            }
        }
        let list = neighbors.join(", ");
        println!("{}: {{{}}}", i + 1, list);
    }
}

/// 약연결성(weak connectivity)을 기준으로 연결 요소들을 출력
/// 방향을 무시한 그래프에서의 연결 요소
pub fn print_weakly_connected_components(matrix: &Matrix) {
    println!("\n=== 연결 요소 (약연결성) ===");
    let n = matrix.len();
    let mut visited = vec![false; n];

    for start in 0..n {
        if visited[start] {
            continue;
        }
        let mut queue = VecDeque::new();
        let mut component = Vec::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            component.push(u);
            // 이웃: 방향 무시 (u->v 또는 v->u 중 하나라도 있으면 연결)
            for v in 0..n {
                if !visited[v] && (matrix[u][v] == 1 || matrix[v][u] == 1) {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }

        let text = component
            .iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{{{}}}", text);
    }
}

/// 행렬을 지정된 제목으로 표준 출력에 행렬 형태로 표시하는 함수
pub fn print_matrix(matrix: &Matrix, title: &str) {
    println!("\n=== {} ===", title);
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

/// 주어진 원소의 동치류를 계산하여 반환하는 함수
/// 동치 관계에서 특정 원소와 같은 동치류에 속하는 모든 원소들의 벡터를 반환
pub fn get_equivalence_class(matrix: &Matrix, element: usize) -> Vec<usize> {
    let mut class = Vec::new();
    for i in 0..matrix.len() {
        if matrix[element][i] == 1 {
            class.push(i);
        }
    }
    class
}

/// 동치 관계의 모든 동치류를 찾아서 출력하는 함수
/// 각 동치류를 [원소] = {원소들} 형식으로 표시하며 중복 출력을 방지
pub fn print_equivalence_classes(matrix: &Matrix) {
    println!("\n=== 동치류 ===");

    let n = matrix.len();
    let mut processed = vec![false; n];

    for i in 0..n {
        if !processed[i] {
            let class = get_equivalence_class(matrix, i);

            let class_str = class.iter()
                .map(|&x| (x + 1).to_string())
                .collect::<Vec<String>>()
                .join(", ");

            println!("[{}] = {{{}}}", i + 1, class_str);

            for &elem in &class {
                processed[elem] = true;
            }
        }
    }
}

/// 동치류의 상세한 분석과 예시를 보여주는 함수
pub fn demonstrate_equivalence_classes(matrix: &Matrix) {
    println!("\n=== 동치류 상세 분석 ===");

    if !super::equivalence::is_equivalence_relation(matrix) {
        println!("동치 관계가 아니므로 동치류 분석을 수행할 수 없습니다.");
        return;
    }

    let n = matrix.len();
    println!("집합: {{1, 2, ..., {}}}", n);

    // 각 원소의 동치류 표시
    println!("\n각 원소의 동치류:");
    for i in 0..n {
        let class = get_equivalence_class(matrix, i);
        let class_str = class.iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("동치류 [{}]: {{{}}}", i + 1, class_str);
    }

    // 기존의 동치류 출력 함수 사용
    println!("\n동치 관계의 파티션 구조:");
    print_equivalence_classes(matrix);
}

/// 각 속성을 개별적으로 상세히 분석하여 출력하는 함수
pub fn analyze_individual_properties(matrix: &Matrix) {
    println!("\n=== 개별 속성 상세 분석 ===");

    let reflexive = super::equivalence::is_reflexive(matrix);
    let symmetric = super::equivalence::is_symmetric(matrix);
    let transitive = super::equivalence::is_transitive(matrix);
    let equivalence = super::equivalence::is_equivalence_relation(matrix);

    println!("반사성 검증:");
    for i in 0..matrix.len() {
        let has_self = matrix[i][i] == 1;
        println!("  R({}, {}) = {} {}", i + 1, i + 1, matrix[i][i], if has_self { "✓" } else { "✗" });
    }
    println!("결과: {}", if reflexive { "반사성 만족 ✓" } else { "반사성 불만족 ✗" });

    println!("\n대칭성 검증:");
    let mut symmetry_violations = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j] != matrix[j][i] {
                symmetry_violations.push((i, j));
            }
        }
    }
    if symmetry_violations.is_empty() {
        println!("  모든 쌍이 대칭적입니다 ✓");
    } else {
        for (i, j) in symmetry_violations {
            println!("  R({}, {}) = {} vs R({}, {}) = {}",
                    i + 1, j + 1, matrix[i][j], j + 1, i + 1, matrix[j][i]);
        }
    }
    println!("결과: {}", if symmetric { "대칭성 만족 ✓" } else { "대칭성 불만족 ✗" });

    println!("\n추이성 검증:");
    let mut transitivity_violations = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            for k in 0..matrix.len() {
                if matrix[i][j] == 1 && matrix[j][k] == 1 && matrix[i][k] != 1 {
                    transitivity_violations.push((i, j, k));
                }
            }
        }
    }
    if transitivity_violations.is_empty() {
        println!("  추이성 위반사항이 없습니다 ✓");
    } else {
        for (i, j, k) in transitivity_violations {
            println!("  R({}, {}) ∧ R({}, {}) = 1, 1이지만 R({}, {}) = {}",
                    i + 1, j + 1, j + 1, k + 1, i + 1, k + 1, matrix[i][k]);
        }
    }
    println!("결과: {}", if transitive { "추이성 만족 ✓" } else { "추이성 불만족 ✗" });

    println!("\n통합 결과: {}", if equivalence {
        "이 관계는 동치 관계입니다! ✓"
    } else {
        "이 관계는 동치 관계가 아닙니다 ✗"
    });
}

/// 관계의 연결성과 추가 속성들을 종합적으로 분석하는 함수
pub fn analyze_relationship_properties(matrix: &Matrix) {
    println!("\n=== 관계 속성 종합 분석 ===");

    let reflexive = super::equivalence::is_reflexive(matrix);
    let symmetric = super::equivalence::is_symmetric(matrix);
    let transitive = super::equivalence::is_transitive(matrix);
    let antisymmetric = super::equivalence::is_antisymmetric(matrix);
    let irreflexive = super::equivalence::is_irreflexive(matrix);
    let connected = super::equivalence::is_connected_relation(matrix);

    println!("기본 동치 관계 속성:");
    println!("  반사성: {}", if reflexive { "✓ 만족" } else { "✗ 불만족" });
    println!("  대칭성: {}", if symmetric { "✓ 만족" } else { "✗ 불만족" });
    println!("  추이성: {}", if transitive { "✓ 만족" } else { "✗ 불만족" });

    println!("\n추가 관계 속성:");
    println!("  반대칭성: {}", if antisymmetric { "✓ 만족" } else { "✗ 불만족" });
    println!("  비반사성: {}", if irreflexive { "✓ 만족" } else { "✗ 불만족" });
    println!("  연결성: {}", if connected { "✓ 만족" } else { "✗ 불만족" });

    println!("\n관계 분류:");
    if reflexive && symmetric && transitive {
        println!("  ✓ 동치 관계 (Equivalence Relation)");
    }
    if antisymmetric && transitive {
        println!("  ✓ 부분 순서 관계 (Partial Order Relation)");
    }
    if irreflexive && symmetric && transitive {
        println!("  ✓ 엄밀 부분 순서 관계 (Strict Partial Order)");
    }
    if irreflexive && antisymmetric && transitive {
        println!("  ✓ 선형 순서 관계 (Linear Order)");
    }
    if reflexive && antisymmetric && transitive && connected {
        println!("  ✓ 전순서 관계 (Total Order)");
    }

}

/// 텍스트 기반 시각화 총괄
pub fn print_text_visualization(matrix: &Matrix) {
    print_adjacency_lists(matrix);
    print_weakly_connected_components(matrix);
}


