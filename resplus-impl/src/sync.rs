use std::borrow::Cow;

use crate::Error;

pub trait ResultChain<I, T, E, D> {
    fn about(self, desc: D) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
    fn about_else(self, f: impl FnOnce() -> D) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
}

impl<I, T, E> ResultChain<I, T, E, &'static str> for std::result::Result<T, E> {
    fn about(self, desc: &'static str) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| Error {
            source: e.into(),
            context: vec![desc.into()],
        })
    }
    fn about_else(self, f: impl FnOnce() -> &'static str) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| Error {
            source: e.into(),
            context: vec![f().into()],
        })
    }
}

impl<I, T, E> ResultChain<I, T, E, String> for std::result::Result<T, E> {
    fn about(self, desc: String) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| Error {
            source: e.into(),
            context: vec![desc.into()],
        })
    }
    fn about_else(self, f: impl FnOnce() -> String) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| Error {
            source: e.into(),
            context: vec![f().into()],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ResultChain;
    use crate as resplus;
    use test_util::*;

    macro_rules! about {
        ($e:expr) => {
            $e.about("source")?
        };
    }
    macro_rules! about_else {
        ($e:expr) => {
            $e.about_else(|| "source")?
        };
    }

    #[test]
    fn about() {
        assert_result!(about!(f0()), "source: Error\n  source");
        assert_result!(about!(f1(1)), "source: Error\n  source");
        assert_result!(about!(f2(1, 1)), "source: Error\n  source");
    }

    #[test]
    fn about_else() {
        assert_result!(about_else!(f0()), "source: Error\n  source");
        assert_result!(about_else!(f1(1)), "source: Error\n  source");
        assert_result!(about_else!(f2(1, 1)), "source: Error\n  source");
    }
}
