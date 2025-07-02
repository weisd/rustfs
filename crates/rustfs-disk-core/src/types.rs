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

//! Core types for disk operations

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::io::{AsyncRead, AsyncWrite};
use uuid::Uuid;

/// Type aliases for file operations
pub type FileReader = Box<dyn AsyncRead + Send + Sync + Unpin>;
pub type FileWriter = Box<dyn AsyncWrite + Send + Sync + Unpin>;

/// Disk location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskLocation {
    pub pool_idx: Option<usize>,
    pub set_idx: Option<usize>,
    pub disk_idx: Option<usize>,
}

impl DiskLocation {
    pub fn valid(&self) -> bool {
        self.pool_idx.is_some() && self.set_idx.is_some() && self.disk_idx.is_some()
    }
}

/// Volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub name: String,
    pub created: Option<OffsetDateTime>,
}

/// Disk information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub used_inodes: u64,
    pub free_inodes: u64,
    pub major: u64,
    pub minor: u64,
    pub nr_requests: u64,
    pub fs_type: String,
    pub root_disk: bool,
    pub healing: bool,
    pub scanning: bool,
    pub endpoint: String,
    pub mount_path: String,
    pub id: String,
    pub rotational: bool,
    pub error: String,
}

/// Disk information options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfoOptions {
    pub disk_id: String,
    pub metrics: bool,
    pub noop: bool,
}

/// Delete options for file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOptions {
    pub recursive: bool,
    pub immediate: bool,
    pub undo_write: bool,
    pub old_data_dir: Option<Uuid>,
}

/// Update metadata options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMetadataOpts {
    pub no_persistence: bool,
}

/// Read options for file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOptions {
    pub incl_free_versions: bool,
    pub read_data: bool,
    pub healing: bool,
}

/// Check parts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPartsResp {
    pub results: Vec<usize>,
}

/// Walk directory options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkDirOptions {
    /// Bucket to scanner
    pub bucket: String,
    /// Directory inside the bucket.
    pub base_dir: String,
    /// Do a full recursive scan.
    pub recursive: bool,
    /// ReportNotFound will return errFileNotFound if all disks reports the BaseDir cannot be found.
    pub report_notfound: bool,
    /// FilterPrefix will only return results with given prefix within folder.
    /// Should never contain a slash.
    pub filter_prefix: Option<String>,
    /// ForwardTo will forward to the given object path.
    pub forward_to: Option<String>,
    /// Limit the number of returned objects if > 0.
    pub limit: i32,
    /// DiskID contains the disk ID of the disk.
    /// Leave empty to not check disk ID.
    pub disk_id: String,
}

/// Rename data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameDataResp {
    pub old_data_dir: Option<Uuid>,
    pub sign: Option<Vec<u8>>,
}

/// Read multiple request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleReq {
    pub bucket: String,
    pub prefix: String,
    pub files: Vec<String>,
    pub max_size: usize,
    pub metadata_only: bool,
    pub abort404: bool,
    pub max_results: usize,
}

/// Read multiple response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleResp {
    pub bucket: String,
    pub prefix: String,
    pub file: String,
    pub exists: bool,
    pub error: String,
    pub data: Vec<u8>,
    pub mod_time: Option<OffsetDateTime>,
}

/// File information with versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfoVersions {
    /// Name of the volume.
    pub volume: String,
    /// Name of the file.
    pub name: String,
    /// Represents the latest mod time of the latest version.
    pub latest_mod_time: Option<OffsetDateTime>,
    pub versions: Vec<FileInfo>,
    pub free_versions: Vec<FileInfo>,
}

impl FileInfoVersions {
    pub fn find_version_index(&self, v: &str) -> Option<usize> {
        self.versions
            .iter()
            .position(|fi| fi.version_id.as_ref().is_some_and(|vid| vid == v))
    }
}

/// Simplified file info placeholder (to be replaced with actual FileInfo from filemeta crate)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub version_id: Option<String>,
    pub size: u64,
    pub mod_time: Option<OffsetDateTime>,
    // Add other necessary fields as needed
}

/// Disk option for initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskOption {
    pub cleanup: bool,
    pub health_check: bool,
}
