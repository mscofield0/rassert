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

#[cfg(test)]
mod tests {
    use crate::rassert;

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
}
