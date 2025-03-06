#[cfg(feature = "async")]
mod fut;
#[cfg(feature = "async")]
pub use fut::FutResultChain;
mod sync;
use std::{borrow::Cow, fmt::Display};
pub use sync::ResultChain;

#[derive(Debug)]
pub struct Error<I> {
    source: I,
    context: Vec<Cow<'static, str>>,
}

impl<I: Display> Display for Error<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "source: {}", self.source)?;
        for c in &self.context {
            write!(f, "\n  {}", c)?;
        }
        Ok(())
    }
}
