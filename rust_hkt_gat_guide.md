## Rust에서 Higher-Kinded Types (HKT)와 관련 개념 정리

### 🧭 1. Higher-Kinded Types란?

> **타입 생성자 자체를 파라미터로 받을 수 있는 추상화 기법**

예: `Option<T>`는 `T`를 받아 구체 타입을 만드는 **타입 생성자**. 이를 `F<T>`로 일반화하여, 타입 생성자 `F`를 파라미터로 받는 추상화를 가능케 함.

---

### ❓ 2. 왜 HKT가 필요한가? (Rust에서는 왜 어려운가?)

다음은 HKT가 필요한 대표적인 세 가지 상황과, HKT 지원 언어와의 비교 예시입니다.

#### 예제 1: 공통 연산 추상화 — `map`

> 다양한 컨테이너(`Option`, `Result`, `Vec`)에 대해 동일한 방식으로 `map`을 적용하고 싶다.

- **Scala**

```scala
trait Functor[F[_]] {
  def map[A, B](fa: F[A])(f: A => B): F[B]
}
```

- **Elixir** (간접적 구현 via Protocols)

```elixir
defprotocol Functor do
  def fmap(fa, f)
end
```

- **Rust (불가능 → GAT로 우회)**

```rust
trait Functor {
    type Wrapped<'a, T>;

    fn fmap<'a, A, B, F>(fa: Self::Wrapped<'a, A>, f: F) -> Self::Wrapped<'a, B>
    where
        F: FnOnce(A) -> B;
}
```

#### 예제 2: 공통 인터페이스로 트레잇 제너릭화 — `Parser[F[_]]`

> 다양한 형태의 파서(`StringParser`, `JsonParser`)를 추상화하고자 한다.

- **Scala**

```scala
trait Parser[F[_]] {
  def parse[A](input: String): F[A]
}
```

- **Rust**

Rust에서는 `F<A>` 형태의 타입 추론이 불가하여 트레잇 내에서 **타입 생성자**를 다루는 것이 불가능.

#### 예제 3: DSL 추상화 — State, Reader Monad 등

> 내부 상태나 외부 의존성 주입을 추상화하는 DSL 구성 시

- **Haskell / Scala**에서는 HKT 기반의 `State[S, A]`, `Reader[R, A]` 타입 생성자를 조합해 매우 간결하게 표현 가능
- **Elixir**: 직접 타입 시스템이 없지만, **동적 디스패치 기반의 Monadic DSL**은 macro/protocol로 가능
- **Rust**: `Box<dyn Fn>` 또는 `impl Trait`/GAT 등으로 복잡하게 우회 필요

---

### 📘 3. 언어별 HKT 지원 비교

| 언어                  | HKT 지원      | 문법                          | 예시                        |
| ------------------- | ----------- | --------------------------- | ------------------------- |
| **Haskell**         | ✅ 완전 지원     | `class Functor f where ...` | `f`는 타입 생성자 (`* -> *`)    |
| **Scala 2**         | ✅ 부분 지원     | `trait Functor[F[_]]`       | `F[_]`는 타입 생성자            |
| **Scala 3 (Dotty)** | ✅ 완전 지원     | `trait Functor[F[_]]`       | Haskell 수준 지원             |
| **Elixir**          | ❌ 타입 시스템 없음 | `defprotocol`, macro 등      | 간접적 추상화 (단, 타입 생성자 개념 없음) |
| **Rust**            | ❌ 미지원       | workaround만 가능              | GAT, trait bounds 등으로 대체  |

---

### 🧪 4. Functor 예제 비교

#### ✅ Haskell

```haskell
class Functor f where
  fmap :: (a -> b) -> f a -> f b
```

#### ✅ Scala

```scala
trait Functor[F[_]] {
  def map[A, B](fa: F[A])(f: A => B): F[B]
}
```

#### ❌ Rust (직접적 불가, GAT 필요)

```rust
#![feature(generic_associated_types)]

trait Functor {
    type Wrapped<'a, T>;

    fn fmap<'a, A, B, F>(fa: Self::Wrapped<'a, A>, f: F) -> Self::Wrapped<'a, B>
    where
        F: FnOnce(A) -> B;
}
```

> Rust는 GAT(Generic Associated Types)를 통해 간접적으로 표현할 수 있지만 제한적

---

### 🛠 5. Rust에서 HKT 대체 기법들

| 기법                     | 설명                              | 예시/라이브러리                    |
| ---------------------- | ------------------------------- | --------------------------- |
| GAT                    | 연관타입에 제네릭 파라미터 허용               | `futures`, `tower` 등        |
| Trait bound trick      | `trait Apply<F: FnOnce(...)>` 등 | `fp-core`, `frunk`          |
| Macro                  | 타입 매개 생성자를 매크로로 우회              | `impl_trait_for_tuples!` 등  |
| Opaque Type/impl Trait | 반환 타입 추상화                       | `async fn`, `impl Future` 등 |
| Enum wrapping          | 구체 타입을 enum으로 래핑                | 패턴매칭 기반 추상화                 |

---

### 🧩 6. 용어 요약

| 용어                     | 한글 번역     | Rust 지원도      | 설명                        |
| ---------------------- | --------- | ------------- | ------------------------- |
| **Higher-Kinded Type** | 고차 종류 타입  | ❌ (직접적 지원 없음) | 타입 생성자를 추상화               |
| **Higher-Ranked Type** | 고차 순위 타입  | ✅             | 수명에 대한 범위 추상화 (`for<'a>`) |
| **GAT**                | 제네릭 연관 타입 | ✅ (unstable)  | 연관 타입에 제네릭 적용             |

---

### 🔧 실전 예제: Rust에서 FP 추상화 구성하기

#### ✅ Result, Option 기반 Functor/Monad 예제

```rust
trait Functor<T> {
    type Output<U>;
    fn fmap<U, F: FnOnce(T) -> U>(self, f: F) -> Self::Output<U>;
}

impl<T> Functor<T> for Option<T> {
    type Output<U> = Option<U>;
    fn fmap<U, F: FnOnce(T) -> U>(self, f: F) -> Self::Output<U> {
        self.map(f)
    }
}

impl<T, E> Functor<T> for Result<T, E> {
    type Output<U> = Result<U, E>;
    fn fmap<U, F: FnOnce(T) -> U>(self, f: F) -> Self::Output<U> {
        self.map(f)
    }
}
```

#### ✅ GAT를 사용하는 Functor 패턴

```rust
#![feature(generic_associated_types)]

trait GATFunctor {
    type F<'a, T>;

    fn fmap<'a, A, B>(input: Self::F<'a, A>, f: fn(A) -> B) -> Self::F<'a, B>;
}
```

#### ✅ HKT 기반 DSL/파서 미니 구현 예제 (의사코드 수준)

```rust
// Rust에서 직접 불가능, 의사코드 형태로 매크로 또는 enum으로 우회

trait Parser<A> {
    fn parse(input: &str) -> Option<A>;
}

struct IntParser;
impl Parser<i32> for IntParser {
    fn parse(input: &str) -> Option<i32> {
        input.parse().ok()
    }
}

fn parse_and_add<P1, P2>(p1: P1, p2: P2, input: &str) -> Option<i32>
where
    P1: Parser<i32>,
    P2: Parser<i32>,
{
    let (a, b) = (p1.parse(input)?, p2.parse(input)?);
    Some(a + b)
}
```

---

