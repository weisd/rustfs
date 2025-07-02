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

//! File system operations for local disk

use rustfs_disk_core::{DiskError, Result};
use std::path::Path;
use tokio::fs;

// File operation flags
pub const O_RDONLY: usize = 0;
pub const O_WRONLY: usize = 1;
pub const O_CREATE: usize = 2;
pub const O_TRUNC: usize = 4;
pub const O_APPEND: usize = 8;

/// Check if file exists and is accessible
pub async fn access(path: impl AsRef<Path>) -> Result<()> {
    match fs::metadata(path).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DiskError::Io(e)),
    }
}

/// Get file metadata
pub async fn lstat(path: impl AsRef<Path>) -> Result<std::fs::Metadata> {
    match fs::metadata(path).await {
        Ok(metadata) => Ok(metadata),
        Err(e) => Err(DiskError::Io(e)),
    }
}

/// Get file metadata (std version)
pub fn lstat_std(path: impl AsRef<Path>) -> Result<std::fs::Metadata> {
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(metadata),
        Err(e) => Err(DiskError::Io(e)),
    }
}

/// Remove a file
pub async fn remove(path: impl AsRef<Path>) -> Result<()> {
    match fs::remove_file(path).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DiskError::Io(e)),
    }
}

/// Remove a directory and all its contents
pub async fn remove_all_std(path: impl AsRef<Path>) -> Result<()> {
    match std::fs::remove_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(DiskError::Io(e)),
    }
}

/// Remove a file (std version)
pub fn remove_std(path: impl AsRef<Path>) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(DiskError::Io(e)),
    }
}

/// Rename a file
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    match fs::rename(from, to).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DiskError::Io(e)),
    }
}
