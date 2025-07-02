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

//! Operating system specific operations

use rustfs_disk_core::{DiskInfo, Result};
use std::path::Path;

/// Get disk usage information
pub fn get_disk_info(path: impl AsRef<Path>) -> Result<DiskInfo> {
    // This is a placeholder implementation
    // In a real implementation, this would use system calls to get actual disk information
    let path_str = path.as_ref().to_string_lossy().to_string();

    Ok(DiskInfo {
        total: 0,
        free: 0,
        used: 0,
        used_inodes: 0,
        free_inodes: 0,
        major: 0,
        minor: 0,
        nr_requests: 0,
        fs_type: "ext4".to_string(), // Default to ext4
        root_disk: false,
        healing: false,
        scanning: false,
        endpoint: path_str.clone(),
        mount_path: path_str,
        id: String::new(),
        rotational: false,
        error: String::new(),
    })
}

/// Check if a path is on the root drive
pub fn is_root_disk(path: impl AsRef<Path>) -> bool {
    // Simplified check - in reality, this would check mount points
    path.as_ref() == Path::new("/")
}

/// Get available disk space
pub fn get_free_space(_path: impl AsRef<Path>) -> Result<u64> {
    // This is a placeholder - real implementation would use statvfs or similar
    Ok(0)
}

/// Check if disk supports O_DIRECT
pub fn supports_direct_io(_path: impl AsRef<Path>) -> bool {
    // Simplified check - assume all Unix-like systems support O_DIRECT
    #[cfg(unix)]
    return true;

    #[cfg(windows)]
    return false;
}
