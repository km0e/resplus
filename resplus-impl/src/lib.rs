#[cfg(feature = "async")]
mod fut;
#[cfg(feature = "async")]
pub use fut::FutResultChain;
mod sync;
use std::{borrow::Cow, fmt::Display};
pub use sync::ResultChain;

#[derive(Debug)]
struct Inner<I> {
    pub source: I,
    pub context: Vec<Cow<'static, str>>,
}

#[derive(Debug)]
pub struct ErrorChain<I> {
    inner: Box<Inner<I>>,
}

impl<I> ErrorChain<I> {
    pub fn new(source: impl Into<I>) -> Self {
        Self {
            inner: Box::new(Inner {
                source: source.into(),
                context: Vec::new(),
            }),
        }
    }
    pub fn with_context(source: impl Into<I>, context: impl Into<Cow<'static, str>>) -> Self {
        Self {
            inner: Box::new(Inner {
                source: source.into(),
                context: vec![context.into()],
            }),
        }
    }
    pub fn append(&mut self, context: impl Into<Cow<'static, str>>) {
        self.inner.context.push(context.into());
    }
    pub fn source(&self) -> &I {
        &self.inner.source
    }
}

impl<I: Display> Display for ErrorChain<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "source: {}", self.inner.source)?;
        for c in self.inner.context.iter() {
            write!(f, "\n  {}", c)?;
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
