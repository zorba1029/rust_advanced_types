# 🦀 Rust Higher-Kinded Types Study

Rust의 고급 타입 시스템 기능들을 학습하고 실습하기 위한 프로젝트다.

- **[ref] (https://medium.com/@trek007/advanced-type-system-features-in-rust-a-deep-dive-into-type-level-programming-473cdcb8c7b5)**


## 📚 프로젝트 개요

이 프로젝트는 **6가지 고급 Rust 타입 시스템 기능**을 시연한다:

1. **🏗️ Higher-Kinded Types** - Associated Type Constructors를 통한 타입 생성자 추상화
2. **🔗 Higher-Ranked Types** - 고급 라이프타임 바운드와 `for<'a>` 문법
3. **🚀 Generic Associated Types (GATs)** - 제네릭 연관 타입과 라이프타임 의존성
4. **🎭 Type-Level State Machines** - PhantomData를 활용한 컴파일 타임 상태 검증
5. **🔢 Const Generics** - 타입 레벨 프로그래밍과 컴파일 타임 크기 계산
6. **🛠️ Type-Safe Builder Pattern** - 복합 타입 시스템 기능을 활용한 실용적 패턴

## 🚀 실행 방법

### 전체 프로젝트 빌드
```bash
cargo build
```

### 개별 테스트 실행
각 기능을 독립적으로 테스트할 수 있다:

```bash
# Higher-Kinded Types (타입 생성자 추상화)
cargo run --bin hkt_test

# Higher-Ranked Types (고급 라이프타임 바운드)
cargo run --bin hrt_test

# Generic Associated Types (제네릭 연관 타입)
cargo run --bin gat_test

# Type-Level State Machines (타입 레벨 상태 머신)
cargo run --bin scheduler_test

# Const Generics (타입 레벨 프로그래밍)
cargo run --bin const_generic_test

# Type-Safe Builder Pattern (복합 타입 시스템 활용)
cargo run --bin typesafe_builder_test
```

### 기본 실행 (사용 안내)
```bash
cargo run
```

## 📖 참조 문서

- **[Higher-Kinded Types 가이드](readme-hkt.md)** - 타입 생성자 추상화와 Scala 비교
- **[Generic Associated Types 가이드](readme-gat.md)** - GATs와 라이프타임 의존성

## 🏗️ 프로젝트 구조

```
src/
├── main.rs                    # 프로젝트 안내 및 메인 엔트리
├── lib.rs                     # 라이브러리 모듈 정의
├── custom_types/              # 고급 타입 시스템 구현들
│   ├── mod.rs                # 모듈 정의
│   ├── container.rs          # Higher-Kinded Types 구현
│   ├── with_lifetime.rs      # Higher-Ranked Types 구현
│   ├── gat.rs                # Generic Associated Types 구현
│   ├── scheduler.rs          # Type-Level State Machines 구현
│   ├── const_generic.rs      # Const Generics 구현
│   └── typesafe_builder.rs   # Type-Safe Builder Pattern 구현
└── bin/                       # 개별 테스트 바이너리들
    ├── hkt_test.rs           # Higher-Kinded Types 테스트
    ├── hrt_test.rs           # Higher-Ranked Types 테스트
    ├── gat_test.rs           # Generic Associated Types 테스트
    ├── scheduler_test.rs     # Type-Level State Machines 테스트
    ├── const_generic_test.rs # Const Generics 테스트
    └── typesafe_builder_test.rs # Type-Safe Builder Pattern 테스트
```

## 🎯 학습 목표

### 1. Higher-Kinded Types (고차 타입)
- 타입 생성자(`Option<T>`, `Result<T, E>`)를 하나의 인터페이스로 추상화
- 함수형 프로그래밍 패턴을 Rust에서 안전하게 구현
- Scala와 Haskell의 HKT와 비교

### 2. Higher-Ranked Types (고차 순위 타입)
- `for<'a>` 문법을 통한 라이프타임 다형성
- 복잡한 라이프타임 관계를 타입 시스템으로 표현
- 컴파일 타임 라이프타임 안전성 보장

### 3. Generic Associated Types (제네릭 연관 타입)
- 연관 타입에 제네릭 매개변수 추가
- 라이프타임 의존적인 타입 관계 표현
- Iterator, Stream 패턴의 고급 활용

### 4. Type-Level State Machines (타입 레벨 상태 머신)
- PhantomData를 활용한 컴파일 타임 상태 검증
- 잘못된 상태 전환을 컴파일 타임에 방지
- Zero-cost 상태 관리

### 5. Const Generics (상수 제네릭)
- 컴파일 타임 크기 계산과 검증
- 타입 레벨에서 배열과 행렬 연산
- 메모리 안전성과 성능 최적화

### 6. Type-Safe Builder Pattern (타입 안전 빌더 패턴)
- 여러 고급 타입 시스템 기능의 실용적 조합
- PhantomData와 제네릭을 활용한 컴파일 타임 상태 추적
- 불완전한 객체 생성을 컴파일 타임에 방지
- 메서드 체이닝과 타입 상태 전환

## 🔥 핵심 개념

### Zero-Cost Abstractions
모든 구현은 **런타임 오버헤드 없이** 컴파일 타임에 검증된다.

### Type Safety
잘못된 타입 조합, 상태 전환, 크기 불일치 등이 **컴파일 타임에 방지**된다.

### Expressiveness
복잡한 도메인 로직을 **타입 시스템으로 표현**하여 버그를 원천 차단한다.

## 🛠️ 요구사항

- Rust 1.65.0 이상 (GATs 안정화 버전)
- Cargo

## 💡 학습 팁

1. **순서대로 학습**: HKT → HRT → GAT → State Machines → Const Generics → Builder Pattern
2. **코드 읽기**: 각 `src/custom_types/` 파일의 주석과 구현을 자세히 읽어보자
3. **실험하기**: 타입을 바꿔보고 컴파일 에러를 관찰해보자
4. **문서 활용**: `readme-hkt.md`와 `readme-gat.md`를 참고하자
5. **통합 이해**: Builder Pattern에서 여러 기능이 어떻게 조합되는지 확인하자

## 🎉 결론

이 프로젝트를 통해 Rust의 타입 시스템이 얼마나 강력하고 표현력 있는지 체험할 수 있다. 각 기능은 메모리 안전성을 보장하면서도 고성능을 달성하는 Rust의 철학을 보여준다.

**"Zero-cost abstractions with compile-time safety"** - 이것이 바로 Rust의 힘이다! 🚀 