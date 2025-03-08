use std::fmt::Display;

use thiserror::Error;

#[macro_export]
macro_rules! assert_result {
    ($r:expr, $e:expr) => {
        assert_eq!(
            || -> Result<(), resplus::ErrorChain<Error>> { $r }()
                .unwrap_err()
                .to_string(),
            $e
        )
    };
}

#[macro_export]
macro_rules! async_assert_result {
    ($r:expr, $e:expr) => {
        assert_eq!(
            async || -> Result<(), resplus::ErrorChain<Error>> { ($r).await }()
                .await
                .unwrap_err()
                .to_string(),
            $e
        )
    };
}
#[derive(Debug)]
pub struct Error1;
impl Display for Error1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}
impl std::error::Error for Error1 {}
#[derive(Debug, Error)]
pub enum Error {
    #[error("Error")]
    E1(#[from] Error1),
}

pub fn f0() -> Result<(), Error1> {
    Err(Error1)
}
pub fn f1(_: i32) -> Result<(), Error1> {
    Err(Error1)
}
pub fn f2(_: i32, _: i32) -> Result<(), Error1> {
    Err(Error1)
}
pub fn f3(_: i32, _: i32, _: i32) -> Result<(), Error1> {
    Err(Error1)
}
pub async fn af0() -> Result<(), Error1> {
    Err(Error1)
}
pub async fn af1(_: i32) -> Result<(), Error1> {
    Err(Error1)
}
pub async fn af2(_: i32, _: i32) -> Result<(), Error1> {
    Err(Error1)
}
pub async fn af3(_: i32, _: i32, _: i32) -> Result<(), Error1> {
    Err(Error1)
}
pub struct Test;
impl Test {
    pub fn f0(&self) -> Result<(), Error1> {
        Err(Error1)
    }
    pub fn f1(&self, _: i32) -> Result<(), Error1> {
        Err(Error1)
    }
    pub fn f2(&self, _: i32, _: i32) -> Result<(), Error1> {
        Err(Error1)
    }
    pub fn f3(&self, _: i32, _: i32, _: i32) -> Result<(), Error1> {
        Err(Error1)
    }
}
