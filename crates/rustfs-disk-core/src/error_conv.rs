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

//! Error conversion utilities for local disk operations

use crate::DiskError;
use std::io;

/// Convert IO error to appropriate disk error
pub fn to_file_error(err: io::Error) -> DiskError {
    match err.kind() {
        io::ErrorKind::NotFound => DiskError::FileNotFound,
        io::ErrorKind::PermissionDenied => DiskError::FileAccessDenied,
        io::ErrorKind::InvalidInput => DiskError::FileNameTooLong,
        _ => DiskError::Io(err),
    }
}

/// Convert IO error to access error
pub fn to_access_error(err: io::Error) -> DiskError {
    match err.kind() {
        io::ErrorKind::PermissionDenied => DiskError::DiskAccessDenied,
        _ => DiskError::Io(err),
    }
}

/// Convert IO error to volume error
pub fn to_volume_error(err: io::Error) -> DiskError {
    match err.kind() {
        io::ErrorKind::NotFound => DiskError::VolumeNotFound,
        io::ErrorKind::PermissionDenied => DiskError::VolumeAccessDenied,
        io::ErrorKind::AlreadyExists => DiskError::VolumeExists,
        _ => DiskError::Io(err),
    }
}

/// Convert IO error to unformatted disk error
pub fn to_unformatted_disk_error(err: io::Error) -> DiskError {
    match err.kind() {
        io::ErrorKind::NotFound => DiskError::UnformattedDisk,
        _ => DiskError::Io(err),
    }
}
