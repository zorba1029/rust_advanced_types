//
// Advanced Higher-Kinded Types: Functor and Monad Patterns
//

// A trait representing a higher-kinded type with one type parameter
pub trait HKT<T> {
    type Higher<U>: HKT<U>;
}

// Functor trait using Higher-Kinded Types
pub trait Functor<T>: HKT<T> {
    fn fmap<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U;
}

// Applicative trait extending Functor
pub trait Applicative<T>: Functor<T> {
    fn pure(value: T) -> Self;
    fn apply<U, F>(self, f: Self::Higher<F>) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U;
}

// Monad trait extending Applicative
pub trait Monad<T>: Applicative<T> {
    fn bind<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> Self::Higher<U>;
}

// Example implementation for Option
impl<T> HKT<T> for Option<T> {
    type Higher<U> = Option<U>;
}

impl<T> Functor<T> for Option<T> {
    fn fmap<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}

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

// Example implementation for Result
impl<T, E> HKT<T> for Result<T, E> {
    type Higher<U> = Result<U, E>;
}

impl<T, E> Functor<T> for Result<T, E> {
    fn fmap<U, F>(self, f: F) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}

impl<T, E> Applicative<T> for Result<T, E> {
    fn pure(value: T) -> Self {
        Ok(value)
    }

    fn apply<U, F>(self, f: Self::Higher<F>) -> Self::Higher<U>
    where
        F: FnOnce(T) -> U,
    {
        match (self, f) {
            (Ok(value), Ok(func)) => Ok(func(value)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

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

// Simplified function that works with Option specifically
pub fn chain_option_operations<T, U, V>(
    m: Option<T>,
    f: impl FnOnce(T) -> Option<U>,
    g: impl FnOnce(U) -> Option<V>,
) -> Option<V> {
    m.bind(f).bind(g)
}

// Simplified function that works with Result specifically
pub fn chain_result_operations<T, U, V, E>(
    m: Result<T, E>,
    f: impl FnOnce(T) -> Result<U, E>,
    g: impl FnOnce(U) -> Result<V, E>,
) -> Result<V, E> {
    m.bind(f).bind(g)
}

// Example usage with Option
pub fn option_example() {
    let result = Option::pure(5)
        .bind(|x| Some(x * 2))
        .bind(|x| Some(x + 1));
    
    println!("Option result: {:?}", result); // Some(11)
}

// Example usage with Result
pub fn result_example() {
    let result: Result<i32, &str> = Result::pure(10)
        .bind(|x| Ok(x / 2))
        .bind(|x| if x > 0 { Ok(x) } else { Err("negative") });
    
    println!("Result: {:?}", result); // Ok(5)
}

// Demonstrate usage with Option
pub fn option_to_string_example(m: Option<i32>) -> Option<String> {
    m.bind(|x| Option::pure(format!("Value: {}", x)))
}

// Demonstrate usage with Result
pub fn result_to_string_example<E>(m: Result<i32, E>) -> Result<String, E> {
    m.bind(|x| Result::pure(format!("Value: {}", x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_monad() {
        let result = Option::pure(42)
            .bind(|x| Some(x * 2))
            .bind(|x| Some(x.to_string()));
        
        assert_eq!(result, Some("84".to_string()));
    }

    #[test]
    fn test_result_monad() {
        let result: Result<String, &str> = Result::pure(21)
            .bind(|x| Ok(x * 2))
            .bind(|x| Ok(x.to_string()));
        
        assert_eq!(result, Ok("42".to_string()));
    }

    #[test]
    fn test_chain_option_operations() {
        let result = chain_option_operations(
            Some(5),
            |x| Some(x * 2),
            |x| Some(x + 1),
        );
        
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_chain_result_operations() {
        let result: Result<i32, &str> = chain_result_operations(
            Ok(5),
            |x| Ok(x * 2),
            |x| Ok(x + 1),
        );
        
        assert_eq!(result, Ok(11));
    }

    #[test]
    fn test_option_applicative() {
        let result = Some(5).apply(Some(|x: i32| x * 2));
        assert_eq!(result, Some(10));
        
        let result2: Option<i32> = None.apply(Some(|x: i32| x * 2));
        assert_eq!(result2, None);
        
        let result3: Option<i32> = Some(5).apply(None::<fn(i32) -> i32>);
        assert_eq!(result3, None);
    }

    #[test]
    fn test_result_applicative() {
        let result: Result<i32, &str> = Ok(10).apply(Ok(|x: i32| x / 2));
        assert_eq!(result, Ok(5));
        
        let result2: Result<i32, &str> = Err("error").apply(Ok(|x: i32| x / 2));
        assert_eq!(result2, Err("error"));
        
        let result3: Result<i32, &str> = Ok(10).apply(Err::<fn(i32) -> i32, &str>("func error"));
        assert_eq!(result3, Err("func error"));
    }
}