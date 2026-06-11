use jni::errors::Error as JniError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
    str::Utf8Error,
};

/// Error type for droid-wrap operations.
#[derive(Debug)]
pub enum DroidWrapError {
    /// A JNI error from the jni crate.
    Jni(JniError),
    /// A string conversion error.
    FromStr(String),
    /// A UTF-8 decoding error.
    Utf8(Utf8Error),
    /// A cloned JNI error (stored as string since jni::Error doesn't implement Clone).
    JniOwned(String),
}

impl Clone for DroidWrapError {
    fn clone(&self) -> Self {
        match self {
            Self::Jni(e) => Self::JniOwned(e.to_string()),
            Self::Utf8(e) => Self::Utf8(e.to_owned()),
            Self::FromStr(s) => Self::FromStr(s.to_owned()),
            Self::JniOwned(s) => Self::JniOwned(s.to_owned()),
        }
    }
}

impl Error for DroidWrapError {}

impl Display for DroidWrapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "DroidWrapError: ")?;
        match self {
            Self::FromStr(e) => Display::fmt(e, f),
            Self::Jni(e) => Display::fmt(e, f),
            Self::Utf8(e) => Display::fmt(e, f),
            Self::JniOwned(s) => Display::fmt(s, f),
        }
    }
}

impl From<JniError> for DroidWrapError {
    fn from(value: JniError) -> Self {
        Self::Jni(value)
    }
}

impl From<Utf8Error> for DroidWrapError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

/// Result type alias for droid-wrap operations.
pub type Result<T> = StdResult<T, DroidWrapError>;
