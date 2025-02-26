/*
 * Copyright (c) 2025. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use jni::errors::{Error as JniError, JniError as JniCallError};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
    str::Utf8Error,
};

/// 错误类型
#[derive(Debug)]
pub enum DroidWrapError {
    Jni(JniError),
    FromStr(String),
    Utf8(Utf8Error),
}

impl Clone for DroidWrapError {
    fn clone(&self) -> Self {
        match self {
            Self::Jni(e) => Self::Jni(match e {
                JniError::WrongJValueType(e, e2) => JniError::WrongJValueType(e, e2),
                JniError::InvalidCtorReturn => JniError::InvalidCtorReturn,
                JniError::InvalidArgList(e) => JniError::InvalidArgList(e.to_owned()),
                JniError::MethodNotFound { name, sig } => JniError::MethodNotFound {
                    name: name.to_owned(),
                    sig: sig.to_owned(),
                },
                JniError::FieldNotFound { name, sig } => JniError::FieldNotFound {
                    name: name.to_owned(),
                    sig: sig.to_owned(),
                },
                JniError::JavaException => JniError::JavaException,
                JniError::JNIEnvMethodNotFound(e) => JniError::JNIEnvMethodNotFound(e),
                JniError::NullPtr(e) => JniError::NullPtr(e),
                JniError::NullDeref(e) => JniError::NullDeref(e),
                JniError::TryLock => JniError::TryLock,
                JniError::JavaVMMethodNotFound(e) => JniError::JavaVMMethodNotFound(e),
                JniError::FieldAlreadySet(e) => JniError::FieldAlreadySet(e.to_owned()),
                JniError::ThrowFailed(e) => JniError::ThrowFailed(e.to_owned()),
                JniError::ParseFailed(e, e2) => JniError::ParseFailed(e.to_owned(), e2.to_owned()),
                JniError::JniCall(e) => JniError::JniCall(match e {
                    JniCallError::Unknown => JniCallError::Unknown,
                    JniCallError::ThreadDetached => JniCallError::ThreadDetached,
                    JniCallError::WrongVersion => JniCallError::WrongVersion,
                    JniCallError::NoMemory => JniCallError::NoMemory,
                    JniCallError::AlreadyCreated => JniCallError::AlreadyCreated,
                    JniCallError::InvalidArguments => JniCallError::InvalidArguments,
                    JniCallError::Other(e) => JniCallError::Other(*e),
                }),
            }),
            Self::Utf8(e) => Self::Utf8(e.to_owned()),
            Self::FromStr(s) => Self::FromStr(s.to_owned())
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

/// 结果类型
pub type Result<T> = StdResult<T, DroidWrapError>;
