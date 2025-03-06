use std::borrow::Cow;

use async_trait::async_trait;

use crate::Error;

#[async_trait]
pub trait FutResultChain<I, T, E, D> {
    async fn about(self, desc: D) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
    async fn about_else(self, f: impl FnOnce() -> D + Send) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
}

#[async_trait]
impl<I, T, E, F> FutResultChain<I, T, E, &'static str> for F
where
    F: Future<Output = std::result::Result<T, E>> + Send,
{
    async fn about(self, desc: &'static str) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.map_err(|e| Error {
            source: e.into(),
            context: vec![desc.into()],
        })
    }
    async fn about_else(self, f: impl FnOnce() -> &'static str + Send) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.map_err(|e| Error {
            source: e.into(),
            context: vec![f().into()],
        })
    }
}

#[async_trait]
impl<I, T, E, F> FutResultChain<I, T, E, String> for F
where
    F: Future<Output = std::result::Result<T, E>> + Send,
{
    async fn about(self, desc: String) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.map_err(|e| Error {
            source: e.into(),
            context: vec![desc.into()],
        })
    }
    async fn about_else(self, f: impl FnOnce() -> String + Send) -> Result<T, Error<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.map_err(|e| Error {
            source: e.into(),
            context: vec![f().into()],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FutResultChain;
    use crate as resplus;
    use test_util::*;

    macro_rules! about {
        ($e:expr) => {
            $e.about("source").await?
        };
    }
    macro_rules! about_else {
        ($e:expr) => {
            $e.about_else(|| "source").await?
        };
    }

    #[tokio::test]
    async fn about() {
        async_assert_result!(about!(af0()), "source: Error\n  source");
        async_assert_result!(about!(af1(1)), "source: Error\n  source");
        async_assert_result!(about!(af2(1, 1)), "source: Error\n  source");
    }

    #[tokio::test]
    async fn about_else() {
        async_assert_result!(about_else!(af0()), "source: Error\n  source");
        async_assert_result!(about_else!(af1(1)), "source: Error\n  source");
        async_assert_result!(about_else!(af2(1, 1)), "source: Error\n  source");
    }
}
