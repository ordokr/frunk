//! GAT-based Type Classes for Higher Kinded Types simulation.
//!
//! Rust does not currently support true Higher Kinded Types (HKT).
//! However, with Generalized Associated Types (GATs), we can simulate them effectively.
//!
//! This module provides the `Functor`, `Apply`, `Applicative`, and `Monad` traits.

/// A Functor is a type constructor that supports mapping a function over its inner value.
pub trait Functor {
    /// The inner value type
    type Inner;

    /// The target type after mapping (the "Kind" applied to a new type)
    type Target<T>;

    /// Maps a function over the inner value.
    fn map<B, F>(self, f: F) -> Self::Target<B>
    where
        F: FnMut(Self::Inner) -> B;
}

/// Apply extends Functor with the ability to apply a wrapped function to a wrapped value.
pub trait Apply: Functor {
    /// Applies a wrapped function to this value.
    ///
    /// The function is wrapped in the same context as the value.
    fn apply<B, F>(self, f: Self::Target<F>) -> Self::Target<B>
    where
        F: FnMut(Self::Inner) -> B;
}

/// Applicative extends Apply with the ability to wrap a pure value.
pub trait Applicative: Apply {
    /// Wraps a value into the context.
    ///
    /// Note: This is `pure` or `return`.
    /// Since traits in Rust are implemented on the concrete type (e.g., `Option<A>`),
    /// this function acts as a factory for the context `Self::Target<T>`.
    fn pure_target<T>(t: T) -> Self::Target<T>;
}

/// Monad extends Applicative with the ability to flatten nested contexts (chaining).
pub trait Monad: Applicative {
    /// Chains an operation that returns a value in the context.
    ///
    /// Also known as `bind` or `>>=`.
    fn flat_map<B, F>(self, f: F) -> Self::Target<B>
    where
        F: FnMut(Self::Inner) -> Self::Target<B>;
}

// Implementations for Option

impl<A> Functor for Option<A> {
    type Inner = A;
    type Target<T> = Option<T>;

    fn map<B, F>(self, f: F) -> Option<B>
    where
        F: FnMut(A) -> B,
    {
        self.map(f)
    }
}

impl<A> Apply for Option<A> {
    fn apply<B, F>(self, f: Option<F>) -> Option<B>
    where
        F: FnMut(A) -> B,
    {
        match (self, f) {
            (Some(a), Some(mut func)) => Some(func(a)),
            _ => None,
        }
    }
}

impl<A> Applicative for Option<A> {
    fn pure_target<T>(t: T) -> Option<T> {
        Some(t)
    }
}

impl<A> Monad for Option<A> {
    fn flat_map<B, F>(self, f: F) -> Option<B>
    where
        F: FnMut(A) -> Option<B>,
    {
        self.and_then(f)
    }
}

// Implementations for Result

impl<A, E> Functor for Result<A, E> {
    type Inner = A;
    type Target<T> = Result<T, E>;

    fn map<B, F>(self, f: F) -> Result<B, E>
    where
        F: FnMut(A) -> B,
    {
        self.map(f)
    }
}

impl<A, E> Apply for Result<A, E> {
    fn apply<B, F>(self, f: Result<F, E>) -> Result<B, E>
    where
        F: FnMut(A) -> B,
    {
        match (self, f) {
            (Ok(a), Ok(mut func)) => Ok(func(a)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn pure_target<T>(t: T) -> Result<T, E> {
        Ok(t)
    }
}

impl<A, E> Monad for Result<A, E> {
    fn flat_map<B, F>(self, f: F) -> Result<B, E>
    where
        F: FnMut(A) -> Result<B, E>,
    {
        self.and_then(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_monad() {
        let val = Some(5);
        let res = val.map(|x| x * 2).flat_map(|x| Some(x + 1));
        assert_eq!(res, Some(11));

        let none: Option<i32> = None;
        let res_none = none.map(|x| x * 2).flat_map(|x| Some(x + 1));
        assert_eq!(res_none, None);
    }

    #[test]
    fn test_result_monad() {
        let val: Result<i32, &str> = Ok(5);
        let res = val.map(|x| x * 2).flat_map(|x| Ok(x + 1));
        assert_eq!(res, Ok(11));

        let err: Result<i32, &str> = Err("oops");
        let res_err = err.map(|x| x * 2).flat_map(|x| Ok(x + 1));
        assert_eq!(res_err, Err("oops"));
    }

    #[test]
    fn test_applicative() {
        let f = Some(|x: i32| x * 2);
        let val = Some(10);
        let res = val.apply(f);
        assert_eq!(res, Some(20));

        let val2: Option<i32> = Option::pure_target(42);
        assert_eq!(val2, Some(42));
    }
}
