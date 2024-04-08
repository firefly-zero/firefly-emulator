use core::fmt::Display;

pub enum Error {
    Runtime(firefly_runtime::Error),
}

impl From<firefly_runtime::Error> for Error {
    fn from(value: firefly_runtime::Error) -> Self {
        Self::Runtime(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}
