// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Error types for disk operations

use std::hash::{Hash, Hasher};
use std::io;
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, DiskError>;

/// Core disk error types
#[derive(Debug, thiserror::Error)]
pub enum DiskError {
    #[error("maximum versions exceeded, please delete few versions to proceed")]
    MaxVersionsExceeded,

    #[error("unexpected error")]
    Unexpected,

    #[error("corrupted format")]
    CorruptedFormat,

    #[error("corrupted backend")]
    CorruptedBackend,

    #[error("unformatted disk error")]
    UnformattedDisk,

    #[error("inconsistent drive found")]
    InconsistentDisk,

    #[error("drive does not support O_DIRECT")]
    UnsupportedDisk,

    #[error("drive path full")]
    DiskFull,

    #[error("disk not a dir")]
    DiskNotDir,

    #[error("disk not found")]
    DiskNotFound,

    #[error("drive still did not complete the request")]
    DiskOngoingReq,

    #[error("drive is part of root drive, will not be used")]
    DriveIsRoot,

    #[error("remote drive is faulty")]
    FaultyRemoteDisk,

    #[error("drive is faulty")]
    FaultyDisk,

    #[error("drive access denied")]
    DiskAccessDenied,

    #[error("file not found")]
    FileNotFound,

    #[error("file version not found")]
    FileVersionNotFound,

    #[error("too many open files, please increase 'ulimit -n'")]
    TooManyOpenFiles,

    #[error("file name too long")]
    FileNameTooLong,

    #[error("volume already exists")]
    VolumeExists,

    #[error("not of regular file type")]
    IsNotRegular,

    #[error("path not found")]
    PathNotFound,

    #[error("volume not found")]
    VolumeNotFound,

    #[error("volume is not empty")]
    VolumeNotEmpty,

    #[error("volume access denied")]
    VolumeAccessDenied,

    #[error("file access denied")]
    FileAccessDenied,

    #[error("file is corrupted")]
    FileCorrupt,

    #[error("short write")]
    ShortWrite,

    #[error("bit-rot hash algorithm is invalid")]
    BitrotHashAlgoInvalid,

    #[error("Rename across devices not allowed, please fix your backend configuration")]
    CrossDeviceLink,

    #[error("less data available than what was requested")]
    LessData,

    #[error("more data was sent than what was advertised")]
    MoreData,

    #[error("outdated XL meta")]
    OutdatedXLMeta,

    #[error("part missing or corrupt")]
    PartMissingOrCorrupt,

    #[error("No healing is required")]
    NoHealRequired,

    #[error("method not allowed")]
    MethodNotAllowed,

    #[error("erasure write quorum")]
    ErasureWriteQuorum,

    #[error("erasure read quorum")]
    ErasureReadQuorum,

    #[error("not implemented: {operation}")]
    NotImplemented { operation: String },

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("custom error: {message}")]
    Custom { message: String },
}

impl DiskError {
    /// Create a custom error with the given message
    pub fn custom(message: impl Into<String>) -> Self {
        DiskError::Custom { message: message.into() }
    }

    /// Create a not implemented error for the given operation
    pub fn not_implemented(operation: impl Into<String>) -> Self {
        DiskError::NotImplemented {
            operation: operation.into(),
        }
    }

    /// Create error from any compatible error type
    pub fn other<E>(error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        DiskError::Io(std::io::Error::other(error))
    }

    /// Check if all errors are "not found" errors
    pub fn is_all_not_found(errs: &[Option<DiskError>]) -> bool {
        for err in errs.iter() {
            if let Some(err) = err {
                if err == &DiskError::FileNotFound || err == &DiskError::FileVersionNotFound {
                    continue;
                }
                return false;
            }
            return false;
        }
        !errs.is_empty()
    }

    /// Check if error is object not found
    pub fn is_err_object_not_found(err: &DiskError) -> bool {
        matches!(err, &DiskError::FileNotFound) || matches!(err, &DiskError::VolumeNotFound)
    }

    /// Check if error is version not found
    pub fn is_err_version_not_found(err: &DiskError) -> bool {
        matches!(err, &DiskError::FileVersionNotFound)
    }
}

impl Clone for DiskError {
    fn clone(&self) -> Self {
        match self {
            DiskError::MaxVersionsExceeded => DiskError::MaxVersionsExceeded,
            DiskError::Unexpected => DiskError::Unexpected,
            DiskError::CorruptedFormat => DiskError::CorruptedFormat,
            DiskError::CorruptedBackend => DiskError::CorruptedBackend,
            DiskError::UnformattedDisk => DiskError::UnformattedDisk,
            DiskError::InconsistentDisk => DiskError::InconsistentDisk,
            DiskError::UnsupportedDisk => DiskError::UnsupportedDisk,
            DiskError::DiskFull => DiskError::DiskFull,
            DiskError::DiskNotDir => DiskError::DiskNotDir,
            DiskError::DiskNotFound => DiskError::DiskNotFound,
            DiskError::DiskOngoingReq => DiskError::DiskOngoingReq,
            DiskError::DriveIsRoot => DiskError::DriveIsRoot,
            DiskError::FaultyRemoteDisk => DiskError::FaultyRemoteDisk,
            DiskError::FaultyDisk => DiskError::FaultyDisk,
            DiskError::DiskAccessDenied => DiskError::DiskAccessDenied,
            DiskError::FileNotFound => DiskError::FileNotFound,
            DiskError::FileVersionNotFound => DiskError::FileVersionNotFound,
            DiskError::TooManyOpenFiles => DiskError::TooManyOpenFiles,
            DiskError::FileNameTooLong => DiskError::FileNameTooLong,
            DiskError::VolumeExists => DiskError::VolumeExists,
            DiskError::IsNotRegular => DiskError::IsNotRegular,
            DiskError::PathNotFound => DiskError::PathNotFound,
            DiskError::VolumeNotFound => DiskError::VolumeNotFound,
            DiskError::VolumeNotEmpty => DiskError::VolumeNotEmpty,
            DiskError::VolumeAccessDenied => DiskError::VolumeAccessDenied,
            DiskError::FileAccessDenied => DiskError::FileAccessDenied,
            DiskError::FileCorrupt => DiskError::FileCorrupt,
            DiskError::ShortWrite => DiskError::ShortWrite,
            DiskError::BitrotHashAlgoInvalid => DiskError::BitrotHashAlgoInvalid,
            DiskError::CrossDeviceLink => DiskError::CrossDeviceLink,
            DiskError::LessData => DiskError::LessData,
            DiskError::MoreData => DiskError::MoreData,
            DiskError::OutdatedXLMeta => DiskError::OutdatedXLMeta,
            DiskError::PartMissingOrCorrupt => DiskError::PartMissingOrCorrupt,
            DiskError::NoHealRequired => DiskError::NoHealRequired,
            DiskError::MethodNotAllowed => DiskError::MethodNotAllowed,
            DiskError::ErasureWriteQuorum => DiskError::ErasureWriteQuorum,
            DiskError::ErasureReadQuorum => DiskError::ErasureReadQuorum,
            DiskError::NotImplemented { operation } => DiskError::NotImplemented {
                operation: operation.clone(),
            },
            DiskError::Io(e) => DiskError::Io(io::Error::new(e.kind(), e.to_string())),
            DiskError::Custom { message } => DiskError::Custom {
                message: message.clone(),
            },
        }
    }
}

impl PartialEq for DiskError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for DiskError {}

impl Hash for DiskError {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

/// File access denied error with context
#[derive(Debug, thiserror::Error)]
#[error("access denied to file: {path}")]
pub struct FileAccessDeniedWithContext {
    pub path: PathBuf,
    #[source]
    pub source: io::Error,
}
