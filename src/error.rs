use core::fmt::Display;

pub enum Error {
    Runtime(firefly_runtime::Error),
    FullID(firefly_runtime::FullIDError),
    MiniFB(minifb::Error),
    Cli(&'static str),
}

impl From<firefly_runtime::FullIDError> for Error {
    fn from(v: firefly_runtime::FullIDError) -> Self {
        Self::FullID(v)
    }
}

impl From<firefly_runtime::Error> for Error {
    fn from(value: firefly_runtime::Error) -> Self {
        Self::Runtime(value)
    }
}

impl From<minifb::Error> for Error {
    fn from(value: minifb::Error) -> Self {
        Self::MiniFB(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Cli(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Runtime(err) => write!(f, "runtime error: {err}"),
            Error::FullID(err) => write!(f, "cannot parse ID: {err}"),
            Error::MiniFB(err) => write!(f, "GUI error: {err}"),
            Error::Cli(err) => write!(f, "CLI error: {err}"),
        }
    }
}
