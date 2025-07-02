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

//! Core traits for disk operations

use crate::{Endpoint, error::Result, types::*};
use async_trait::async_trait;
use bytes::Bytes;
use std::path::PathBuf;
use tokio::io::AsyncWrite;
use uuid::Uuid;

/// Core disk API trait that all disk implementations must follow
#[async_trait]
pub trait DiskAPI: std::fmt::Debug + Send + Sync + 'static {
    /// Returns a string representation of the disk
    fn to_string(&self) -> String;

    /// Check if the disk is online
    async fn is_online(&self) -> bool;

    /// Check if the disk is local
    fn is_local(&self) -> bool;

    /// Get the hostname of the disk
    fn host_name(&self) -> String;

    /// Get the endpoint of the disk
    fn endpoint(&self) -> Endpoint;

    /// Close the disk connection
    async fn close(&self) -> Result<()>;

    /// Get the disk ID
    async fn get_disk_id(&self) -> Result<Option<Uuid>>;

    /// Set the disk ID
    async fn set_disk_id(&self, id: Option<Uuid>) -> Result<()>;

    /// Get the disk path
    fn path(&self) -> PathBuf;

    /// Get the disk location information
    fn get_disk_location(&self) -> DiskLocation;

    // Volume operations
    /// Create a volume
    async fn make_volume(&self, volume: &str) -> Result<()>;

    /// Create multiple volumes
    async fn make_volumes(&self, volumes: Vec<&str>) -> Result<()>;

    /// List all volumes
    async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>;

    /// Get volume information
    async fn stat_volume(&self, volume: &str) -> Result<VolumeInfo>;

    /// Delete a volume
    async fn delete_volume(&self, volume: &str) -> Result<()>;

    // Directory operations
    /// Walk through directory and write results to output
    async fn walk_dir<W: AsyncWrite + Unpin + Send>(&self, opts: WalkDirOptions, wr: &mut W) -> Result<()>;

    // Metadata operations
    /// Delete a file version
    async fn delete_version(
        &self,
        volume: &str,
        path: &str,
        fi: FileInfo,
        force_del_marker: bool,
        opts: DeleteOptions,
    ) -> Result<()>;

    /// Delete multiple file versions
    async fn delete_versions(
        &self,
        volume: &str,
        versions: Vec<FileInfoVersions>,
        opts: DeleteOptions,
    ) -> Result<Vec<Option<crate::error::DiskError>>>;

    /// Delete multiple paths
    async fn delete_paths(&self, volume: &str, paths: &[String]) -> Result<()>;

    /// Write metadata for a file
    async fn write_metadata(&self, org_volume: &str, volume: &str, path: &str, fi: FileInfo) -> Result<()>;

    /// Update metadata for a file
    async fn update_metadata(&self, volume: &str, path: &str, fi: FileInfo, opts: &UpdateMetadataOpts) -> Result<()>;

    /// Read a specific version of a file
    async fn read_version(
        &self,
        org_volume: &str,
        volume: &str,
        path: &str,
        version_id: &str,
        opts: &ReadOptions,
    ) -> Result<FileInfo>;

    /// Read XL metadata
    async fn read_xl(&self, volume: &str, path: &str, read_data: bool) -> Result<Vec<u8>>;

    /// Rename data from source to destination
    async fn rename_data(
        &self,
        src_volume: &str,
        src_path: &str,
        file_info: FileInfo,
        dst_volume: &str,
        dst_path: &str,
    ) -> Result<RenameDataResp>;

    // File operations
    /// List directory contents
    async fn list_dir(&self, origvolume: &str, volume: &str, dir_path: &str, count: i32) -> Result<Vec<String>>;

    /// Read a file and return a reader
    async fn read_file(&self, volume: &str, path: &str) -> Result<FileReader>;

    /// Read a file stream with offset and length
    async fn read_file_stream(&self, volume: &str, path: &str, offset: usize, length: usize) -> Result<FileReader>;

    /// Open a file for appending
    async fn append_file(&self, volume: &str, path: &str) -> Result<FileWriter>;

    /// Create a new file
    async fn create_file(&self, origvolume: &str, volume: &str, path: &str, file_size: i64) -> Result<FileWriter>;

    /// Rename a file
    async fn rename_file(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str) -> Result<()>;

    /// Rename a part file with metadata
    async fn rename_part(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str, meta: Bytes) -> Result<()>;

    /// Delete a file
    async fn delete(&self, volume: &str, path: &str, opt: DeleteOptions) -> Result<()>;

    /// Verify file integrity
    async fn verify_file(&self, volume: &str, path: &str, fi: &FileInfo) -> Result<CheckPartsResp>;

    /// Check parts of a file
    async fn check_parts(&self, volume: &str, path: &str, fi: &FileInfo) -> Result<CheckPartsResp>;

    /// Read multiple files
    async fn read_multiple(&self, req: ReadMultipleReq) -> Result<Vec<ReadMultipleResp>>;

    /// Write all data to a file
    async fn write_all(&self, volume: &str, path: &str, data: Bytes) -> Result<()>;

    /// Read all data from a file
    async fn read_all(&self, volume: &str, path: &str) -> Result<Bytes>;

    /// Get disk information
    async fn disk_info(&self, opts: &DiskInfoOptions) -> Result<DiskInfo>;
}

/// Utility functions for error handling
pub fn conv_part_err_to_int(err: &Option<crate::error::DiskError>) -> usize {
    match err {
        None => crate::constants::CHECK_PART_SUCCESS,
        Some(e) => match e {
            crate::error::DiskError::DiskNotFound => crate::constants::CHECK_PART_DISK_NOT_FOUND,
            crate::error::DiskError::VolumeNotFound => crate::constants::CHECK_PART_VOLUME_NOT_FOUND,
            crate::error::DiskError::FileNotFound => crate::constants::CHECK_PART_FILE_NOT_FOUND,
            crate::error::DiskError::FileCorrupt => crate::constants::CHECK_PART_FILE_CORRUPT,
            _ => crate::constants::CHECK_PART_UNKNOWN,
        },
    }
}

/// Check if there are any part errors
pub fn has_part_err(part_errs: &[usize]) -> bool {
    part_errs.iter().any(|&err| err != crate::constants::CHECK_PART_SUCCESS)
}
