use std::borrow::Cow;

use crate::ErrorChain;

pub trait ResultChain<I, T, E, D> {
    fn about(self, desc: D) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
    fn about_else(self, f: impl FnOnce() -> D) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
        D: Into<Cow<'static, str>>;
}

impl<I, T, E> ResultChain<I, T, E, &'static str> for std::result::Result<T, E> {
    fn about(self, desc: &'static str) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| ErrorChain::with_context(e, desc))
    }
    fn about_else(self, f: impl FnOnce() -> &'static str) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| ErrorChain::with_context(e, f()))
    }
}

impl<I, T, E> ResultChain<I, T, E, String> for std::result::Result<T, E> {
    fn about(self, desc: String) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| ErrorChain::with_context(e, desc))
    }
    fn about_else(self, f: impl FnOnce() -> String) -> Result<T, ErrorChain<I>>
    where
        Self: Sized,
        E: Into<I>,
    {
        self.map_err(|e| ErrorChain::with_context(e, f()))
    }
}
//
// pub trait ResultChainAttach<T, D> {
//     fn attach(self, desc: D) -> Result<T, ErrorChain<I>>
//     where
//         Self: Sized,
//         D: Into<Cow<'static, str>>;
//     fn attach_else(self, f: impl FnOnce() -> D) -> Result<T, ErrorChain<I>>
//     where
//         Self: Sized,
//         D: Into<Cow<'static, str>>;
// }

#[cfg(test)]
mod tests {
    use super::ResultChain;
    use crate as resplus;
    use crate::tests::{about, about_else};
    use test_util::*;

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
    //
    // #[test]
    // fn attach() {
    //     assert_result!(attach!(about!(f0())), "source: Error\n  source\n  attach");
    //     assert_result!(attach!(about!(f1(1))), "source: Error\n  source\n  attach");
    //     assert_result!(
    //         attach!(about!(f2(1, 1))),
    //         "source: Error\n  source\n  attach"
    //     );
    // }
    //
    // #[test]
    // fn attach_else() {
    //     assert_result!(
    //         attach_else!(about_else!(f0())),
    //         "source: Error\n  source\n  attach"
    //     );
    //     assert_result!(
    //         attach_else!(about_else!(f1(1))),
    //         "source: Error\n  source\n  attach"
    //     );
    //     assert_result!(
    //         attach_else!(about_else!(f2(1, 1))),
    //         "source: Error\n  source\n  attach"
    //     );
    // }
}
