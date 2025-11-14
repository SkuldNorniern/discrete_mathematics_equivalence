# Discrete Mathematics: Equivalence Relations Analyzer

이산수학 과제 - 동치 관계 분석기

관계 행렬을 분석하여 동치 관계 여부를 판별하고, 다양한 속성과 폐포 연산을 수행하는 Rust 프로그램입니다.

## 주요 기능 (Features)

### 동치 관계 판별 (Equivalence Relation Analysis)
- **반사성 (Reflexivity)**: 모든 원소가 자신과 관계를 맺는지 확인
- **대칭성 (Symmetry)**: 관계가 대칭적인지 확인
- **추이성 (Transitivity)**: 관계가 추이적인지 확인

### 추가 속성 분석 (Additional Properties)
- **반대칭성 (Antisymmetry)**: 관계가 반대칭적인지 확인
- **비반사성 (Irreflexivity)**: 관계가 비반사적인지 확인
- **연결성 (Connectedness)**: 관계가 연결적인지 확인

### 폐포 연산 (Closure Operations)
- **반사 폐포**: 관계가 반사성을 만족하도록 만듦
- **대칭 폐포**: 관계가 대칭성을 만족하도록 만듦
- **추이 폐포**: Floyd-Warshall 알고리즘으로 추이성을 만족하도록 만듦

### 시각화 및 분석 (Visualization & Analysis)
- **그래프 시각화**: 인접 리스트 형태로 관계 표시
- **연결 요소 분석**: 약연결성 기반 연결 요소 찾기
- **동치류 분석**: 각 동치류의 원소들을 그룹화하여 표시
- **관계 밀도 분석**: 관계의 밀집도 계산

### 동적 크기 지원 (Dynamic Size Support)
- **행렬 크기**: 2×2 ~ 10×10 범위 지원
- **유연한 입력**: 사용자가 원하는 크기의 행렬 분석 가능


## 설치 및 실행 (Installation & Usage)

### Rust 설치
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 기본 실행
```bash
cargo run
```

### 릴리즈 모드로 실행
```bash
cargo run --release
```
