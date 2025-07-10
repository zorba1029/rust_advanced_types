# Rust Monad 패턴 상세 가이드

## test_option_monad 함수 분석

### 1. 함수의 목적
이 함수는 Option 타입에 대한 Monad 패턴의 구현을 테스트하고 시연하는 함수다. Monad는 함수형 프로그래밍의 핵심 개념으로, 값을 컨테이너에 감싸고 연속적인 변환을 우아하게 처리할 수 있게 해준다.

### 2. Monad trait 구현 살펴보기

먼저 핵심이 되는 Monad trait 구현을 보자:

```rust
// Monad trait 정의
pub trait Monad<T>: Applicative<T> {
    fn bind<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> Self::Higher<U>;
}

// Option에 대한 Monad 구현
impl<T> Monad<T> for Option<T> {
    fn bind<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> Self::Higher<U>,
    {
        match self {
            Some(value) => f(value),  // 값이 있으면 함수 적용
            None => None,             // None이면 그대로 None 반환
        }
    }
}
```

### 3. test_option_monad 함수 동작 분석

#### 3.1 Option::pure 사용
```rust
let value = Option::pure(42);
println!("Option::pure(42) = {:?}", value);  // Some(42) 출력
```

`pure` 메서드는 Applicative trait에 정의되어 있고, 값을 Option으로 감싸는 역할을 한다:
```rust
fn pure(value: T) -> Self {
    Some(value)
}
```

#### 3.2 bind 메서드 사용
```rust
let result = value.bind(|x| Some(x * 2));
println!("bind(|x| Some(x * 2)) = {:?}", result);  // Some(84) 출력
```

`bind`는 다음과 같이 동작한다:
1. `value`는 `Some(42)`
2. `bind`는 내부 값 42를 꺼낸다
3. 람다 함수 `|x| Some(x * 2)`에 42를 전달
4. 결과로 `Some(84)` 반환

#### 3.3 연속된 bind 체이닝
```rust
let final_result = Option::pure(5)
    .bind(|x| Some(x + 10))    // 5 + 10 = 15
    .bind(|x| Some(x * 2))     // 15 * 2 = 30
    .bind(|x| Some(format!("결과: {}", x)));  // "결과: 30"
println!("연속된 bind 결과: {:?}", final_result);  // Some("결과: 30")
```

각 단계별 동작:
1. `Option::pure(5)` → `Some(5)`
2. 첫 번째 bind: `Some(5)` → `Some(15)`
3. 두 번째 bind: `Some(15)` → `Some(30)`
4. 세 번째 bind: `Some(30)` → `Some("결과: 30")`

#### 3.4 None 처리
```rust
let none_result: Option<i32> = None;
let none_bind = none_result.bind(|x| Some(x * 2));
println!("None.bind() = {:?}", none_bind);  // None 출력
```

`bind` 구현에서 None인 경우 함수를 호출하지 않고 바로 None을 반환한다. 이것이 Monad의 핵심 기능 중 하나로, 에러나 없는 값을 자동으로 전파한다.

## 4. 전체 Monad 구현 코드

### 4.1 Higher-Kinded Type (HKT) trait
```rust
// A trait representing a higher-kinded type with one type parameter
pub trait HKT<T> {
    type Higher<U>: HKT<U>;
}
```

### 4.2 Functor trait
```rust
// Functor trait using Higher-Kinded Types
pub trait Functor<T>: HKT<T> {
    fn fmap<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U;
}
```

### 4.3 Applicative trait
```rust
// Applicative trait extending Functor
pub trait Applicative<T>: Functor<T> {
    fn pure(value: T) -> Self;
    fn apply<U, F>(self, f: Self::Higher<F>) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U;
}
```

### 4.4 Option에 대한 전체 구현
```rust
// HKT 구현
impl<T> HKT<T> for Option<T> {
    type Higher<U> = Option<U>;
}

// Functor 구현
impl<T> Functor<T> for Option<T> {
    fn fmap<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}

// Applicative 구현
impl<T> Applicative<T> for Option<T> {
    fn pure(value: T) -> Self {
        Some(value)
    }

    fn apply<U, F>(self, f: Self::Higher<F>) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U,
    {
        match (self, f) {
            (Some(value), Some(func)) => Some(func(value)),
            _ => None,
        }
    }
}

// Monad 구현
impl<T> Monad<T> for Option<T> {
    fn bind<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> Self::Higher<U>,
    {
        match self {
            Some(value) => f(value),
            None => None,
        }
    }
}
```

## 5. 실제 사용 예제

### 5.1 monad_test.rs의 main 함수에서 호출
```rust
match input.trim() {
    "1" => test_option_monad(),  // 사용자가 1을 입력하면 실행
    // ...
}
```

### 5.2 복잡한 연산 체이닝 (test_complex_operations)
```rust
let complex_option = Some(3)
    .bind(|x| Some(x * x))  // 제곱: 3 * 3 = 9
    .bind(|x| if x > 5 { Some(x) } else { None })  // 조건부 필터: 9 > 5이므로 Some(9)
    .bind(|x| Some(format!("결과: {}", x)));  // 문자열 변환: Some("결과: 9")
println!("복합 Option 연산: {:?}", complex_option);
```

### 5.3 실용적인 사용자 데이터 처리 (test_practical_examples)
```rust
fn create_user(name: String, age: i32, email: String) -> Result<User, &'static str> {
    validate_age(age)
        .bind(|valid_age| validate_email(&email).bind(|valid_email| {
            Ok(User {
                name,
                age: valid_age,
                email: valid_email,
            })
        }))
}
```

이 예제에서는:
1. 먼저 나이를 검증한다
2. 나이가 유효하면 이메일을 검증한다
3. 둘 다 유효하면 User 객체를 생성한다
4. 어느 단계에서든 에러가 발생하면 자동으로 전파된다

### 5.4 체인 연산 헬퍼 함수
```rust
pub fn chain_option_operations<T, U, V>(
    m: Option<T>,
    f: impl FnOnce(T) -> Option<U>,
    g: impl FnOnce(U) -> Option<V>,
) -> Option<V> {
    m.bind(f).bind(g)
}
```

사용 예:
```rust
let result = chain_option_operations(
    Some(5),
    |x| if x > 0 { Some(x * 2) } else { None },  // 5 > 0이므로 Some(10)
    |x| if x < 20 { Some(x + 10) } else { None }  // 10 < 20이므로 Some(20)
);
println!("chain_option_operations 결과: {:?}", result);  // Some(20)
```

## 6. Result에 대한 Monad 구현

Option과 유사하게 Result도 Monad로 구현되어 있다:

```rust
impl<T, E> Monad<T> for Result<T, E> {
    fn bind<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> Self::Higher<U>,
    {
        match self {
            Ok(value) => f(value),
            Err(e) => Err(e),
        }
    }
}
```

사용 예제:
```rust
let result: Result<String, &str> = Result::pure(21)
    .bind(|x| Ok(x * 2))  // 21 * 2 = 42
    .bind(|x| Ok(x.to_string()));  // "42"
```

## 7. Monad 패턴의 장점

1. **에러 전파 자동화**: None이나 에러가 발생하면 이후 연산을 건너뛴다
2. **보일러플레이트 감소**: 매번 None 체크를 하지 않아도 된다
3. **가독성 향상**: 연속된 변환을 명확하게 표현할 수 있다
4. **타입 안전성**: 컴파일 타임에 타입 체크가 된다
5. **함수 합성**: 작은 함수들을 조합해서 복잡한 로직을 구성할 수 있다

## 8. 에러 처리 예제

```rust
fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("0으로 나눌 수 없습니다")
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: i32) -> Result<f64, &'static str> {
    if x < 0 {
        Err("음수의 제곱근은 계산할 수 없습니다")
    } else {
        Ok((x as f64).sqrt())
    }
}

// 성공적인 연산
let success_result = Ok(100)
    .bind(|x| safe_divide(x, 4))  // 100 / 4 = 25
    .bind(|x| safe_sqrt(x))       // sqrt(25) = 5.0
    .bind(|x| Ok(format!("최종 결과: {:.2}", x)));
println!("성공적인 연산: {:?}", success_result);  // Ok("최종 결과: 5.00")

// 에러가 발생하는 연산
let error_result = Ok(100)
    .bind(|x| safe_divide(x, 0))  // 0으로 나누기 에러
    .bind(|x| safe_sqrt(x))       // 이 연산은 실행되지 않음
    .bind(|x| Ok(format!("최종 결과: {:.2}", x)));
println!("에러가 발생하는 연산: {:?}", error_result);  // Err("0으로 나눌 수 없습니다")
```

## 9. Applicative 패턴 사용

Applicative는 함수를 컨테이너 안에 넣고 적용할 수 있게 해준다:

```rust
// Option Applicative 예제
let value = Some(10);
let func = Some(|x: i32| x * 2);
let result = value.apply(func);
println!("Some(10).apply(Some(|x| x * 2)) = {:?}", result);  // Some(20)

// 복합 Applicative 연산
let add_func = Some(|x: i32| move |y: i32| x + y);
let val1 = Some(5);
let val2 = Some(3);

let partial_add = val1.apply(add_func);  // Some(|y| 5 + y)
let add_result = val2.apply(partial_add);  // Some(8)
println!("Some(5 + 3) using applicative = {:?}", add_result);
```

## 10. Rust의 기존 메서드와의 관계

이 Monad 패턴은 Rust의 기존 메서드들과 유사한 기능을 제공한다:
- `bind` ≈ `and_then`
- `fmap` ≈ `map`
- `?` 연산자도 비슷한 에러 전파 기능을 제공

하지만 이 구현은 더 일반적이고 추상화된 형태로, 다양한 타입에 대해 일관된 인터페이스를 제공한다. 