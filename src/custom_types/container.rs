//
// Associated Type Constructors
// and Higher-Kinded Types
//

// Fitst, let's defined a trait for container-like types
pub trait Container {
    // Associated type for the contained value
    type Item;

    // Associated type constructor for transforming the container
    type Mapped<U>: Container<Item = U>;

    fn map<U, F: FnMut(&Self::Item) -> U>(self, f: F) -> Self::Mapped<U>;
}

// Implementing Container for Option
impl<T> Container for Option<T> {
    type Item = T;
    type Mapped<U> = Option<U>;

    fn map<U, F: FnMut(&Self::Item) -> U>(self, mut f: F) -> Self::Mapped<U> {
        self.map(|x| f(&x))
    }
}

// Implementing Container for Result
impl<T, E> Container for Result<T, E> {
    type Item = T;
    type Mapped<U> = Result<U, E>;

    fn map<U, F: FnMut(&Self::Item) -> U>(self, mut f: F) -> Self::Mapped<U> {
        self.map(|x| f(&x))
    }
}
