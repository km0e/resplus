#[cfg(feature = "async")]
mod fut;
#[cfg(feature = "async")]
pub use fut::FutResultChain;
mod sync;
use std::{borrow::Cow, fmt::Display};
pub use sync::ResultChain;

#[derive(Debug)]
pub struct ErrorChain<I> {
    pub source: I,
    context: Vec<Cow<'static, str>>,
}

impl<I> ErrorChain<I> {
    pub fn new(source: impl Into<I>) -> Self {
        Self {
            source: source.into(),
            context: Vec::new(),
        }
    }
}

impl<I: Display> Display for ErrorChain<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "source: {}", self.source)?;
        for c in &self.context {
            write!(f, "\n  {}", c)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! define_error {
    ($error:ty, $source:ty$(, $t:ty)*) => {
        impl From<$source> for ErrorChain {
            fn from(value: $source) -> Self {
                ErrorChain(resplus::ErrorChain::new(value))
            }
        }
        define_error!($error$(, $t)*);
    };
    ($error:ty) => {
        impl From<$error> for ErrorChain {
            fn from(value: $error) -> Self {
                ErrorChain(resplus::ErrorChain::new(value))
            }
        }

        impl From<resplus::ErrorChain<$error>> for ErrorChain {
            fn from(value: resplus::ErrorChain<$error>) -> Self {
                ErrorChain(value)
            }
        }

        #[derive(Debug)]
        pub struct ErrorChain(resplus::ErrorChain<$error>);

        impl std::fmt::Display for ErrorChain {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::error::Error for ErrorChain {}
    };
}

#[cfg(test)]
mod tests {
    macro_rules! about {
        ($e:expr) => {
            ($e).about("source")
        };
    }
    pub(crate) use about;
    macro_rules! about_else {
        ($e:expr) => {
            ($e).about_else(|| "source")
        };
    }
    pub(crate) use about_else;
}
