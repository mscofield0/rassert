/// Helper macro to return an Err if the expression fails.
///
/// # Example
/// ```rust
/// use rassert_rs::rassert;
///
/// enum MyError {
///     SaulBadman,
/// }
///
/// fn test() -> Result<i32, MyError> {
///     rassert!(42 != 42, MyError::SaulBadman);
///     Ok(72)
/// }
///
/// fn main() {
///     assert!(test().is_err());
/// }
/// ```
#[macro_export]
macro_rules! rassert {
    ($expr: expr, $err: expr) => {
        let _ = if $expr {
            Ok(())
        } else {
            Err($err)
        }?;
    };
}

/// Helper macro to cleanly execute an expression and return out of the function if it fails.
///
/// # Example
/// ```rust
/// use rassert_rs::rassert_notify;
///
/// fn main() {
///     rassert_notify!(42 != 42, println!("Yikes"));
///     // Prints 'Yikes'
/// }
/// ```
#[macro_export]
macro_rules! rassert_notify {
    ($expr: expr, $notify: expr) => {
        if !$expr {
            $notify;
            return;
        }
    };
}

/// Helper macro to cleanly execute an expression and continue a loop if it fails.
///
/// # Example
/// ```rust
/// use rassert_rs::rassert_notify_continue;
///
/// fn main() {
///     let mut i = 1;
///     while i != 5 {
///         i += 1;
///         rassert_notify_continue!(42 != 42, println!("Yikes"));
///         // Prints 'Yikes' and continues the loop
///     }
/// }
/// ```
#[macro_export]
macro_rules! rassert_notify_continue {
    ($expr: expr, $notify: expr) => {
        if !$expr {
            $notify;
            continue;
        }
    };
}

/// Helper macro to cleanly execute an expression and break a loop if it fails.
///
/// # Example
/// ```rust
/// use rassert_rs::rassert_notify_break;
///
/// fn main() {
///     let mut i = 1;
///     while i != 5 {
///         rassert_notify_break!(42 != 42, println!("Yikes"));
///         // Prints 'Yikes' and breaks the loop
///         i += 1;
///     }
///     assert_eq!(i, 1);
/// }
/// ```
#[macro_export]
macro_rules! rassert_notify_break {
    ($expr: expr, $notify: expr) => {
        if !$expr {
            $notify;
            break;
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{rassert, rassert_notify};

    struct TestError;

    fn driver(f: impl Fn() -> Result<(), TestError>) -> Result<(), TestError> {
        f()
    }

    #[test]
    fn check_ok() {
        let result = driver(|| {
            rassert!(1 == 1, TestError);

            Ok(())
        });

        assert!(result.is_ok());
    }

    #[test]
    fn check_err() {
        let result = driver(|| {
            rassert!(1 != 1, TestError);

            Ok(())
        });

        assert!(result.is_err());
    }

    #[allow(unused_assignments)]
    #[test]
    fn check_notify() {
        let mut var = 72;
        rassert_notify!(42 != 42, var = 42);
        assert_eq!(var, 42);
    }

    #[allow(unused_assignments)]
    #[test]
    fn check_notify_continue() {
        let mut var = 72;
        while var != 42 {
            rassert_notify_continue!(42 != 42, var = 42);
        }
        assert_eq!(var, 42);
    }

    #[allow(unused_assignments)]
    #[test]
    fn check_notify_break() {
        let mut var = 72;
        loop {
            rassert_notify_break!(42 != 42, var = 42);
        }
        assert_eq!(var, 42);
    }
}
