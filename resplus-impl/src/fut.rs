use std::borrow::Cow;

use async_trait::async_trait;

use crate::{ErrorChain, ResultChain};

#[async_trait]
pub trait FutResultChain<I, T, E, D> {
    async fn about(self, desc: D) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
    async fn about_else(self, f: impl FnOnce() -> D + Send) -> Result<T, ErrorChain<I>>
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
    async fn about(self, desc: &'static str) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.about(desc)
    }
    async fn about_else(self, f: impl FnOnce() -> &'static str + Send) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.about_else(f)
    }
}

#[async_trait]
impl<I, T, E, F> FutResultChain<I, T, E, String> for F
where
    F: Future<Output = std::result::Result<T, E>> + Send,
{
    async fn about(self, desc: String) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.about(desc)
    }
    async fn about_else(self, f: impl FnOnce() -> String + Send) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.await.about_else(f)
    }
}

#[cfg(test)]
mod tests {
    use super::FutResultChain;
    use crate as resplus;
    use crate::tests::{about, about_else};
    use test_util::*;

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
    //
    // #[tokio::test]
    // async fn attach() {
    //     async_assert_result!(attach!(about!(af0())), "source: Error\n  source\n  attach");
    //     async_assert_result!(attach!(about!(af1(1))), "source: Error\n  source\n  attach");
    //     async_assert_result!(
    //         attach!(about!(af2(1, 1))),
    //         "source: Error\n  source\n  attach"
    //     );
    // }
    //
    // #[tokio::test]
    // async fn attach_else() {
    //     async_assert_result!(
    //         attach_else!(about!(af0())),
    //         "source: Error\n  source\n  attach"
    //     );
    //     async_assert_result!(
    //         attach_else!(about!(af1(1))),
    //         "source: Error\n  source\n  attach"
    //     );
    //     async_assert_result!(
    //         attach_else!(about!(af2(1, 1))),
    //         "source: Error\n  source\n  attach"
    //     );
    // }
}
