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

//! Local disk implementation

use crate::format::FormatV3;
use async_trait::async_trait;
use bytes::Bytes;
use rustfs_disk_core::{
    CheckPartsResp, DeleteOptions, DiskError, DiskInfo, DiskInfoOptions, DiskLocation, Endpoint, FileInfo, FileInfoVersions,
    FileReader, FileWriter, ReadMultipleReq, ReadMultipleResp, ReadOptions, RenameDataResp, Result, UpdateMetadataOpts,
    VolumeInfo, WalkDirOptions, constants::*, traits::DiskAPI,
};
use std::path::PathBuf;
use tokio::io::AsyncWrite;
use uuid::Uuid;

/// Local disk implementation
#[derive(Debug)]
pub struct LocalDisk {
    /// Root path of the disk
    pub root_path: PathBuf,
    /// Endpoint information
    pub endpoint: Endpoint,
    /// Disk location in the set
    pub location: DiskLocation,
    /// Format information
    pub format: Option<FormatV3>,
    /// Whether the disk has been formatted
    pub formatted: bool,
}

impl LocalDisk {
    /// Create a new local disk instance
    pub async fn new(endpoint: &Endpoint, _cleanup: bool) -> Result<Self> {
        let root_path = PathBuf::from(endpoint.get_file_path());

        // Create directory if it doesn't exist
        if let Err(e) = tokio::fs::create_dir_all(&root_path).await {
            return Err(DiskError::Io(e));
        }

        Ok(LocalDisk {
            root_path,
            endpoint: endpoint.clone(),
            location: DiskLocation {
                pool_idx: Some(endpoint.pool_idx as usize),
                set_idx: Some(endpoint.set_idx as usize),
                disk_idx: Some(endpoint.disk_idx as usize),
            },
            format: None,
            formatted: false,
        })
    }
}

#[async_trait]
impl DiskAPI for LocalDisk {
    fn to_string(&self) -> String {
        format!("LocalDisk({})", self.root_path.display())
    }

    async fn is_online(&self) -> bool {
        self.root_path.exists()
    }

    fn is_local(&self) -> bool {
        true
    }

    fn host_name(&self) -> String {
        "localhost".to_string()
    }

    fn endpoint(&self) -> Endpoint {
        self.endpoint.clone()
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }

    async fn get_disk_id(&self) -> Result<Option<Uuid>> {
        // TODO: Read from format.json
        Ok(None)
    }

    async fn set_disk_id(&self, _id: Option<Uuid>) -> Result<()> {
        // TODO: Write to format.json
        Ok(())
    }

    fn path(&self) -> PathBuf {
        self.root_path.clone()
    }

    fn get_disk_location(&self) -> DiskLocation {
        self.location.clone()
    }

    // Volume operations
    async fn make_volume(&self, volume: &str) -> Result<()> {
        let volume_path = self.root_path.join(volume);
        tokio::fs::create_dir_all(volume_path).await.map_err(DiskError::Io)
    }

    async fn make_volumes(&self, volumes: Vec<&str>) -> Result<()> {
        for volume in volumes {
            self.make_volume(volume).await?;
        }
        Ok(())
    }

    async fn list_volumes(&self) -> Result<Vec<VolumeInfo>> {
        let mut volumes = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.root_path).await.map_err(DiskError::Io)?;

        while let Some(entry) = entries.next_entry().await.map_err(DiskError::Io)? {
            if entry.file_type().await.map_err(DiskError::Io)?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    volumes.push(VolumeInfo {
                        name: name.to_string(),
                        created: None, // TODO: Get creation time
                    });
                }
            }
        }

        Ok(volumes)
    }

    async fn stat_volume(&self, volume: &str) -> Result<VolumeInfo> {
        let volume_path = self.root_path.join(volume);
        if !volume_path.exists() {
            return Err(DiskError::VolumeNotFound);
        }

        Ok(VolumeInfo {
            name: volume.to_string(),
            created: None, // TODO: Get creation time
        })
    }

    async fn delete_volume(&self, volume: &str) -> Result<()> {
        let volume_path = self.root_path.join(volume);
        tokio::fs::remove_dir_all(volume_path).await.map_err(DiskError::Io)
    }

    // Directory operations
    async fn walk_dir<W: AsyncWrite + Unpin + Send>(&self, _opts: WalkDirOptions, _wr: &mut W) -> Result<()> {
        // TODO: Implement directory walking
        Err(DiskError::MethodNotAllowed)
    }

    // Metadata operations
    async fn delete_version(
        &self,
        _volume: &str,
        _path: &str,
        _fi: FileInfo,
        _force_del_marker: bool,
        _opts: DeleteOptions,
    ) -> Result<()> {
        // TODO: Implement version deletion
        Err(DiskError::MethodNotAllowed)
    }

    async fn delete_versions(
        &self,
        _volume: &str,
        _versions: Vec<FileInfoVersions>,
        _opts: DeleteOptions,
    ) -> Result<Vec<Option<DiskError>>> {
        // TODO: Implement multiple version deletion
        Err(DiskError::MethodNotAllowed)
    }

    async fn delete_paths(&self, _volume: &str, _paths: &[String]) -> Result<()> {
        // TODO: Implement path deletion
        Err(DiskError::MethodNotAllowed)
    }

    async fn write_metadata(&self, _org_volume: &str, _volume: &str, _path: &str, _fi: FileInfo) -> Result<()> {
        // TODO: Implement metadata writing
        Err(DiskError::MethodNotAllowed)
    }

    async fn update_metadata(&self, _volume: &str, _path: &str, _fi: FileInfo, _opts: &UpdateMetadataOpts) -> Result<()> {
        // TODO: Implement metadata updating
        Err(DiskError::MethodNotAllowed)
    }

    async fn read_version(
        &self,
        _org_volume: &str,
        _volume: &str,
        _path: &str,
        _version_id: &str,
        _opts: &ReadOptions,
    ) -> Result<FileInfo> {
        // TODO: Implement version reading
        Err(DiskError::MethodNotAllowed)
    }

    async fn read_xl(&self, _volume: &str, _path: &str, _read_data: bool) -> Result<Vec<u8>> {
        // TODO: Implement XL metadata reading
        Err(DiskError::MethodNotAllowed)
    }

    async fn rename_data(
        &self,
        _src_volume: &str,
        _src_path: &str,
        _file_info: FileInfo,
        _dst_volume: &str,
        _dst_path: &str,
    ) -> Result<RenameDataResp> {
        // TODO: Implement data renaming
        Err(DiskError::MethodNotAllowed)
    }

    // File operations
    async fn list_dir(&self, _origvolume: &str, volume: &str, dir_path: &str, _count: i32) -> Result<Vec<String>> {
        let path = self.root_path.join(volume).join(dir_path);
        let mut entries = Vec::new();
        let mut dir = tokio::fs::read_dir(path).await.map_err(DiskError::Io)?;

        while let Some(entry) = dir.next_entry().await.map_err(DiskError::Io)? {
            if let Some(name) = entry.file_name().to_str() {
                entries.push(name.to_string());
            }
        }

        Ok(entries)
    }

    async fn read_file(&self, volume: &str, path: &str) -> Result<FileReader> {
        let file_path = self.root_path.join(volume).join(path);
        let file = tokio::fs::File::open(file_path).await.map_err(DiskError::Io)?;
        Ok(Box::new(file))
    }

    async fn read_file_stream(&self, volume: &str, path: &str, _offset: usize, _length: usize) -> Result<FileReader> {
        // TODO: Implement stream reading with offset/length
        self.read_file(volume, path).await
    }

    async fn append_file(&self, volume: &str, path: &str) -> Result<FileWriter> {
        let file_path = self.root_path.join(volume).join(path);
        let file = tokio::fs::OpenOptions::new()
            .append(true)
            .open(file_path)
            .await
            .map_err(DiskError::Io)?;
        Ok(Box::new(file))
    }

    async fn create_file(&self, _origvolume: &str, volume: &str, path: &str, _file_size: i64) -> Result<FileWriter> {
        let file_path = self.root_path.join(volume).join(path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(DiskError::Io)?;
        }

        let file = tokio::fs::File::create(file_path).await.map_err(DiskError::Io)?;
        Ok(Box::new(file))
    }

    async fn rename_file(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str) -> Result<()> {
        let src_file_path = self.root_path.join(src_volume).join(src_path);
        let dst_file_path = self.root_path.join(dst_volume).join(dst_path);

        tokio::fs::rename(src_file_path, dst_file_path).await.map_err(DiskError::Io)
    }

    async fn rename_part(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str, _meta: Bytes) -> Result<()> {
        // For now, just rename the file
        self.rename_file(src_volume, src_path, dst_volume, dst_path).await
    }

    async fn delete(&self, volume: &str, path: &str, _opt: DeleteOptions) -> Result<()> {
        let file_path = self.root_path.join(volume).join(path);
        tokio::fs::remove_file(file_path).await.map_err(DiskError::Io)
    }

    async fn verify_file(&self, _volume: &str, _path: &str, _fi: &FileInfo) -> Result<CheckPartsResp> {
        // TODO: Implement file verification
        Ok(CheckPartsResp {
            results: vec![CHECK_PART_SUCCESS],
        })
    }

    async fn check_parts(&self, _volume: &str, _path: &str, _fi: &FileInfo) -> Result<CheckPartsResp> {
        // TODO: Implement parts checking
        Ok(CheckPartsResp {
            results: vec![CHECK_PART_SUCCESS],
        })
    }

    async fn read_multiple(&self, _req: ReadMultipleReq) -> Result<Vec<ReadMultipleResp>> {
        // TODO: Implement multiple file reading
        Ok(Vec::new())
    }

    async fn write_all(&self, volume: &str, path: &str, data: Bytes) -> Result<()> {
        let file_path = self.root_path.join(volume).join(path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(DiskError::Io)?;
        }

        tokio::fs::write(file_path, data).await.map_err(DiskError::Io)
    }

    async fn read_all(&self, volume: &str, path: &str) -> Result<Bytes> {
        let file_path = self.root_path.join(volume).join(path);
        let data = tokio::fs::read(file_path).await.map_err(DiskError::Io)?;
        Ok(Bytes::from(data))
    }

    async fn disk_info(&self, _opts: &DiskInfoOptions) -> Result<DiskInfo> {
        crate::os::get_disk_info(&self.root_path)
    }
}
