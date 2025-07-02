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

//! Adapter module for transitioning from old disk system to new disk crates
//!
//! This module provides adapters and converters to gradually migrate from the
//! existing disk implementation to the new modular disk crates.

use std::sync::Arc;

use async_trait::async_trait;
use rustfs_disk_core::{
    DiskAPI as NewDiskAPI, DiskError as NewDiskError, DiskOption as NewDiskOption, Endpoint as NewEndpoint, Result as NewResult,
};
use rustfs_disk_local::LocalDisk as NewLocalDisk;
use rustfs_disk_remote::RemoteDisk as NewRemoteDisk;

use crate::disk::{
    DiskAPI as OldDiskAPI, DiskOption as OldDiskOption, DiskStore as OldDiskStore, Endpoint as OldEndpoint, Error as OldError,
    Result as OldResult,
};
use crate::heal::{
    data_scanner::ShouldSleepFn,
    data_usage_cache::{DataUsageCache, DataUsageEntry},
    heal_commands::{HealScanMode, HealingTracker},
};

/// Adapter that wraps new disk implementations to work with old interfaces
#[derive(Debug)]
pub enum DiskAdapter {
    Local(NewLocalDisk),
    Remote(NewRemoteDisk),
}

impl DiskAdapter {
    /// Create a new local disk using the new implementation
    pub async fn new_local(ep: &OldEndpoint, opt: &OldDiskOption) -> OldResult<Self> {
        let new_ep = convert_endpoint(ep)?;

        let local_disk = NewLocalDisk::new(&new_ep, opt.cleanup)
            .await
            .map_err(convert_new_error_to_old)?;

        Ok(DiskAdapter::Local(local_disk))
    }

    /// Create a new remote disk using the new implementation
    pub async fn new_remote(ep: &OldEndpoint, opt: &OldDiskOption) -> OldResult<Self> {
        let new_ep = convert_endpoint(ep)?;
        let new_opt = convert_disk_option(opt);

        let remote_disk = NewRemoteDisk::new(&new_ep, &new_opt)
            .await
            .map_err(convert_new_error_to_old)?;

        Ok(DiskAdapter::Remote(remote_disk))
    }
}

#[async_trait]
impl OldDiskAPI for DiskAdapter {
    fn to_string(&self) -> String {
        match self {
            DiskAdapter::Local(local) => local.to_string(),
            DiskAdapter::Remote(remote) => remote.to_string(),
        }
    }

    async fn is_online(&self) -> bool {
        match self {
            DiskAdapter::Local(local) => local.is_online().await,
            DiskAdapter::Remote(remote) => remote.is_online().await,
        }
    }

    fn is_local(&self) -> bool {
        match self {
            DiskAdapter::Local(local) => local.is_local(),
            DiskAdapter::Remote(remote) => remote.is_local(),
        }
    }

    fn host_name(&self) -> String {
        match self {
            DiskAdapter::Local(local) => local.host_name(),
            DiskAdapter::Remote(remote) => remote.host_name(),
        }
    }

    fn endpoint(&self) -> OldEndpoint {
        let new_ep = match self {
            DiskAdapter::Local(local) => local.endpoint(),
            DiskAdapter::Remote(remote) => remote.endpoint(),
        };
        convert_new_endpoint_to_old(&new_ep)
    }

    async fn close(&self) -> OldResult<()> {
        match self {
            DiskAdapter::Local(local) => local.close().await.map_err(convert_new_error_to_old),
            DiskAdapter::Remote(remote) => remote.close().await.map_err(convert_new_error_to_old),
        }
    }

    async fn get_disk_id(&self) -> OldResult<Option<uuid::Uuid>> {
        match self {
            DiskAdapter::Local(local) => local.get_disk_id().await.map_err(convert_new_error_to_old),
            DiskAdapter::Remote(remote) => remote.get_disk_id().await.map_err(convert_new_error_to_old),
        }
    }

    async fn set_disk_id(&self, id: Option<uuid::Uuid>) -> OldResult<()> {
        match self {
            DiskAdapter::Local(local) => local.set_disk_id(id).await.map_err(convert_new_error_to_old),
            DiskAdapter::Remote(remote) => remote.set_disk_id(id).await.map_err(convert_new_error_to_old),
        }
    }

    fn path(&self) -> std::path::PathBuf {
        match self {
            DiskAdapter::Local(local) => local.path(),
            DiskAdapter::Remote(remote) => remote.path(),
        }
    }

    fn get_disk_location(&self) -> crate::disk::DiskLocation {
        let new_location = match self {
            DiskAdapter::Local(local) => local.get_disk_location(),
            DiskAdapter::Remote(remote) => remote.get_disk_location(),
        };
        crate::disk::DiskLocation {
            pool_idx: new_location.pool_idx,
            set_idx: new_location.set_idx,
            disk_idx: new_location.disk_idx,
        }
    }

    // Volume operations
    async fn make_volume(&self, volume: &str) -> OldResult<()> {
        match self {
            DiskAdapter::Local(local) => local.make_volume(volume).await.map_err(convert_new_error_to_old),
            DiskAdapter::Remote(remote) => remote.make_volume(volume).await.map_err(convert_new_error_to_old),
        }
    }

    async fn make_volumes(&self, volumes: Vec<&str>) -> OldResult<()> {
        match self {
            DiskAdapter::Local(local) => local.make_volumes(volumes).await.map_err(convert_new_error_to_old),
            DiskAdapter::Remote(remote) => remote.make_volumes(volumes).await.map_err(convert_new_error_to_old),
        }
    }

    async fn list_volumes(&self) -> OldResult<Vec<crate::disk::VolumeInfo>> {
        let new_volumes = match self {
            DiskAdapter::Local(local) => local.list_volumes().await.map_err(convert_new_error_to_old)?,
            DiskAdapter::Remote(remote) => remote.list_volumes().await.map_err(convert_new_error_to_old)?,
        };
        Ok(new_volumes.into_iter().map(convert_volume_info).collect())
    }

    async fn stat_volume(&self, volume: &str) -> OldResult<crate::disk::VolumeInfo> {
        let new_volume = match self {
            DiskAdapter::Local(local) => local.stat_volume(volume).await.map_err(convert_new_error_to_old)?,
            DiskAdapter::Remote(remote) => remote.stat_volume(volume).await.map_err(convert_new_error_to_old)?,
        };
        Ok(convert_volume_info(new_volume))
    }

    async fn delete_volume(&self, volume: &str) -> OldResult<()> {
        match self {
            DiskAdapter::Local(local) => local.delete_volume(volume).await.map_err(convert_new_error_to_old),
            DiskAdapter::Remote(remote) => remote.delete_volume(volume).await.map_err(convert_new_error_to_old),
        }
    }

    async fn disk_info(&self, opts: &crate::disk::DiskInfoOptions) -> OldResult<crate::disk::DiskInfo> {
        let new_opts = convert_disk_info_options(opts);
        let new_info = match self {
            DiskAdapter::Local(local) => local.disk_info(&new_opts).await.map_err(convert_new_error_to_old)?,
            DiskAdapter::Remote(remote) => remote.disk_info(&new_opts).await.map_err(convert_new_error_to_old)?,
        };
        Ok(convert_disk_info(new_info))
    }

    // TODO: Implement all other methods...
    // For now, we'll return "not implemented" errors for methods we haven't converted yet

    async fn walk_dir<W: tokio::io::AsyncWrite + Unpin + Send>(
        &self,
        _opts: crate::disk::WalkDirOptions,
        _wr: &mut W,
    ) -> OldResult<()> {
        Err(OldError::other("walk_dir adapter not implemented yet"))
    }

    async fn delete_version(
        &self,
        _volume: &str,
        _path: &str,
        _fi: crate::disk::FileInfo,
        _force_del_marker: bool,
        _opts: crate::disk::DeleteOptions,
    ) -> OldResult<()> {
        Err(OldError::other("delete_version adapter not implemented yet"))
    }

    async fn delete_versions(
        &self,
        _volume: &str,
        _versions: Vec<crate::disk::FileInfoVersions>,
        _opts: crate::disk::DeleteOptions,
    ) -> OldResult<Vec<Option<OldError>>> {
        Err(OldError::other("delete_versions adapter not implemented yet"))
    }

    async fn delete_paths(&self, _volume: &str, _paths: &[String]) -> OldResult<()> {
        Err(OldError::other("delete_paths adapter not implemented yet"))
    }

    async fn write_metadata(&self, _org_volume: &str, _volume: &str, _path: &str, _fi: crate::disk::FileInfo) -> OldResult<()> {
        Err(OldError::other("write_metadata adapter not implemented yet"))
    }

    async fn update_metadata(
        &self,
        _volume: &str,
        _path: &str,
        _fi: crate::disk::FileInfo,
        _opts: &crate::disk::UpdateMetadataOpts,
    ) -> OldResult<()> {
        Err(OldError::other("update_metadata adapter not implemented yet"))
    }

    async fn read_version(
        &self,
        _org_volume: &str,
        _volume: &str,
        _path: &str,
        _version_id: &str,
        _opts: &crate::disk::ReadOptions,
    ) -> OldResult<crate::disk::FileInfo> {
        Err(OldError::other("read_version adapter not implemented yet"))
    }

    async fn read_xl(&self, _volume: &str, _path: &str, _read_data: bool) -> OldResult<crate::disk::RawFileInfo> {
        Err(OldError::other("read_xl adapter not implemented yet"))
    }

    async fn rename_data(
        &self,
        _src_volume: &str,
        _src_path: &str,
        _file_info: crate::disk::FileInfo,
        _dst_volume: &str,
        _dst_path: &str,
    ) -> OldResult<crate::disk::RenameDataResp> {
        Err(OldError::other("rename_data adapter not implemented yet"))
    }

    async fn list_dir(&self, _origvolume: &str, _volume: &str, _dir_path: &str, _count: i32) -> OldResult<Vec<String>> {
        Err(OldError::other("list_dir adapter not implemented yet"))
    }

    async fn read_file(&self, _volume: &str, _path: &str) -> OldResult<crate::disk::FileReader> {
        Err(OldError::other("read_file adapter not implemented yet"))
    }

    async fn read_file_stream(
        &self,
        _volume: &str,
        _path: &str,
        _offset: usize,
        _length: usize,
    ) -> OldResult<crate::disk::FileReader> {
        Err(OldError::other("read_file_stream adapter not implemented yet"))
    }

    async fn append_file(&self, _volume: &str, _path: &str) -> OldResult<crate::disk::FileWriter> {
        Err(OldError::other("append_file adapter not implemented yet"))
    }

    async fn create_file(
        &self,
        _origvolume: &str,
        _volume: &str,
        _path: &str,
        _file_size: i64,
    ) -> OldResult<crate::disk::FileWriter> {
        Err(OldError::other("create_file adapter not implemented yet"))
    }

    async fn rename_file(&self, _src_volume: &str, _src_path: &str, _dst_volume: &str, _dst_path: &str) -> OldResult<()> {
        Err(OldError::other("rename_file adapter not implemented yet"))
    }

    async fn rename_part(
        &self,
        _src_volume: &str,
        _src_path: &str,
        _dst_volume: &str,
        _dst_path: &str,
        _meta: bytes::Bytes,
    ) -> OldResult<()> {
        Err(OldError::other("rename_part adapter not implemented yet"))
    }

    async fn delete(&self, _volume: &str, _path: &str, _opt: crate::disk::DeleteOptions) -> OldResult<()> {
        Err(OldError::other("delete adapter not implemented yet"))
    }

    async fn verify_file(
        &self,
        _volume: &str,
        _path: &str,
        _fi: &crate::disk::FileInfo,
    ) -> OldResult<crate::disk::CheckPartsResp> {
        Err(OldError::other("verify_file adapter not implemented yet"))
    }

    async fn check_parts(
        &self,
        _volume: &str,
        _path: &str,
        _fi: &crate::disk::FileInfo,
    ) -> OldResult<crate::disk::CheckPartsResp> {
        Err(OldError::other("check_parts adapter not implemented yet"))
    }

    async fn read_multiple(&self, _req: crate::disk::ReadMultipleReq) -> OldResult<Vec<crate::disk::ReadMultipleResp>> {
        Err(OldError::other("read_multiple adapter not implemented yet"))
    }

    async fn write_all(&self, _volume: &str, _path: &str, _data: bytes::Bytes) -> OldResult<()> {
        Err(OldError::other("write_all adapter not implemented yet"))
    }

    async fn read_all(&self, _volume: &str, _path: &str) -> OldResult<bytes::Bytes> {
        Err(OldError::other("read_all adapter not implemented yet"))
    }

    async fn ns_scanner(
        &self,
        _cache: &DataUsageCache,
        _updates: tokio::sync::mpsc::Sender<DataUsageEntry>,
        _scan_mode: HealScanMode,
        _we_sleep: ShouldSleepFn,
    ) -> OldResult<DataUsageCache> {
        Err(OldError::other("ns_scanner adapter not implemented yet"))
    }

    async fn healing(&self) -> Option<HealingTracker> {
        None
    }
}

/// Convert old endpoint to new endpoint format
fn convert_endpoint(old_ep: &OldEndpoint) -> OldResult<NewEndpoint> {
    Ok(NewEndpoint {
        url: old_ep.url.clone(),
        is_local: old_ep.is_local,
        pool_idx: old_ep.pool_idx,
        set_idx: old_ep.set_idx,
        disk_idx: old_ep.disk_idx,
    })
}

/// Convert new endpoint to old endpoint format
fn convert_new_endpoint_to_old(new_ep: &NewEndpoint) -> OldEndpoint {
    OldEndpoint {
        url: new_ep.url.clone(),
        is_local: new_ep.is_local,
        pool_idx: new_ep.pool_idx,
        set_idx: new_ep.set_idx,
        disk_idx: new_ep.disk_idx,
    }
}

/// Convert old disk option to new disk option format
fn convert_disk_option(old_opt: &OldDiskOption) -> NewDiskOption {
    NewDiskOption {
        cleanup: old_opt.cleanup,
        health_check: old_opt.health_check,
    }
}

/// Convert new error to old error format
fn convert_new_error_to_old(new_err: NewDiskError) -> OldError {
    match new_err {
        NewDiskError::VolumeNotFound => OldError::VolumeNotFound,
        NewDiskError::DiskNotFound => OldError::DiskNotFound,
        NewDiskError::FileNotFound => OldError::FileNotFound,
        NewDiskError::FileCorrupt => OldError::FileCorrupt,
        NewDiskError::NotImplemented { operation } => OldError::other(format!("Not implemented: {}", operation)),
        _ => OldError::other(format!("Unknown error: {}", new_err)),
    }
}

/// Convert new volume info to old volume info format
fn convert_volume_info(new_vol: rustfs_disk_core::VolumeInfo) -> crate::disk::VolumeInfo {
    crate::disk::VolumeInfo {
        name: new_vol.name,
        created: new_vol.created,
    }
}

/// Convert old disk info options to new disk info options format
fn convert_disk_info_options(old_opts: &crate::disk::DiskInfoOptions) -> rustfs_disk_core::DiskInfoOptions {
    rustfs_disk_core::DiskInfoOptions {
        disk_id: old_opts.disk_id.clone(),
        metrics: old_opts.metrics,
        noop: old_opts.noop,
    }
}

/// Convert new disk info to old disk info format
fn convert_disk_info(new_info: rustfs_disk_core::DiskInfo) -> crate::disk::DiskInfo {
    crate::disk::DiskInfo {
        total: new_info.total,
        free: new_info.free,
        used: new_info.used,
        used_inodes: new_info.used_inodes,
        free_inodes: new_info.free_inodes,
        major: new_info.major,
        minor: new_info.minor,
        nr_requests: new_info.nr_requests,
        fs_type: new_info.fs_type,
        root_disk: new_info.root_disk,
        healing: new_info.healing,
        scanning: new_info.scanning,
        endpoint: new_info.endpoint,
        mount_path: new_info.mount_path,
        id: new_info.id,
        rotational: new_info.rotational,
        metrics: crate::disk::DiskMetrics::default(), // TODO: Convert properly
        error: new_info.error,
    }
}

/// Factory function that creates a disk using the new implementation but returns old interface
pub async fn new_disk_adapter(ep: &OldEndpoint, opt: &OldDiskOption) -> OldResult<OldDiskStore> {
    let adapter = if ep.is_local {
        DiskAdapter::new_local(ep, opt).await?
    } else {
        DiskAdapter::new_remote(ep, opt).await?
    };

    // Create a proper disk enum variant
    match adapter {
        DiskAdapter::Local(_) => {
            // For now, we can't directly use the adapter because Disk::Local expects LocalDisk
            // We'll need to return an error or implement a different approach
            Err(OldError::other("DiskAdapter to Disk conversion not implemented yet"))
        }
        DiskAdapter::Remote(_) => {
            // Same issue with Remote
            Err(OldError::other("DiskAdapter to Disk conversion not implemented yet"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_endpoint_conversion() {
        let url = url::Url::from_str("http://localhost:9000/data").unwrap();
        let old_ep = OldEndpoint {
            url: url.clone(),
            is_local: true,
            pool_idx: 0,
            set_idx: 1,
            disk_idx: 2,
        };

        let new_ep = convert_endpoint(&old_ep).unwrap();
        assert_eq!(new_ep.url, url);
        assert_eq!(new_ep.is_local, true);
        assert_eq!(new_ep.pool_idx, 0);
        assert_eq!(new_ep.set_idx, 1);
        assert_eq!(new_ep.disk_idx, 2);

        let converted_back = convert_new_endpoint_to_old(&new_ep);
        assert_eq!(converted_back.url, old_ep.url);
        assert_eq!(converted_back.is_local, old_ep.is_local);
        assert_eq!(converted_back.pool_idx, old_ep.pool_idx);
        assert_eq!(converted_back.set_idx, old_ep.set_idx);
        assert_eq!(converted_back.disk_idx, old_ep.disk_idx);
    }

    #[test]
    fn test_disk_option_conversion() {
        let old_opt = OldDiskOption {
            cleanup: true,
            health_check: false,
        };

        let new_opt = convert_disk_option(&old_opt);
        assert_eq!(new_opt.cleanup, true);
        assert_eq!(new_opt.health_check, false);
    }
}
