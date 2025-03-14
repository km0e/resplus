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
    pub context: Vec<Cow<'static, str>>,
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
        for (i, c) in self.context.iter().enumerate() {
            write!(f, "\n {} {}", i, c)?;
        }
        Ok(())
    }
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
