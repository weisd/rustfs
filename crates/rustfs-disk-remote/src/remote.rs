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

use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;
use futures::lock::Mutex;
use http::{HeaderMap, HeaderValue, Method, header::CONTENT_TYPE};
use protos::{
    node_service_time_out_client,
    proto_gen::node_service::{
        ListVolumesRequest, MakeVolumeRequest, MakeVolumesRequest, 
        StatVolumeRequest, DeleteVolumeRequest,
    },
};

use rustfs_disk_core::{
    CheckPartsResp, DeleteOptions, DiskInfo, DiskInfoOptions, DiskLocation, DiskOption, Endpoint, FileInfo, 
    FileInfoVersions, FileReader, FileWriter, ReadMultipleReq, ReadMultipleResp, ReadOptions, RenameDataResp, 
    Result, UpdateMetadataOpts, VolumeInfo, WalkDirOptions, DiskError,
    traits::DiskAPI,
};
use rustfs_rio::{HttpReader, HttpWriter};
use tokio::io::AsyncWrite;
use tracing::info;
use uuid::Uuid;

/// Remote disk implementation for distributed RustFS
#[derive(Debug)]
pub struct RemoteDisk {
    pub id: Mutex<Option<Uuid>>,
    pub addr: String,
    pub url: url::Url,
    pub root: PathBuf,
    endpoint: Endpoint,
}

impl RemoteDisk {
    pub async fn new(ep: &Endpoint, _opt: &DiskOption) -> Result<Self> {
        let root = PathBuf::from(ep.get_file_path());
        let addr = if let Some(port) = ep.url.port() {
            format!("{}://{}:{}", ep.url.scheme(), ep.url.host_str().unwrap(), port)
        } else {
            format!("{}://{}", ep.url.scheme(), ep.url.host_str().unwrap())
        };
        Ok(Self {
            id: Mutex::new(None),
            addr,
            url: ep.url.clone(),
            root,
            endpoint: ep.clone(),
        })
    }

    /// Build authentication headers for HTTP requests
    fn build_auth_headers(&self, _url: &str, _method: &Method, headers: &mut HeaderMap) {
        // TODO: Implement proper authentication
        // This is a placeholder - actual auth implementation needed
        headers.insert("Authorization", HeaderValue::from_static("Bearer dummy-token"));
    }
}

#[async_trait]
impl DiskAPI for RemoteDisk {
    #[tracing::instrument(skip(self))]
    fn to_string(&self) -> String {
        self.endpoint.to_string()
    }

    #[tracing::instrument(skip(self))]
    async fn is_online(&self) -> bool {
        // TODO: Check connection status
        if node_service_time_out_client(&self.addr).await.is_ok() {
            return true;
        }
        false
    }

    #[tracing::instrument(skip(self))]
    fn is_local(&self) -> bool {
        false
    }

    #[tracing::instrument(skip(self))]
    fn host_name(&self) -> String {
        self.endpoint.host_port()
    }

    #[tracing::instrument(skip(self))]
    fn endpoint(&self) -> Endpoint {
        self.endpoint.clone()
    }

    #[tracing::instrument(skip(self))]
    async fn close(&self) -> Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_disk_id(&self) -> Result<Option<Uuid>> {
        Ok(*self.id.lock().await)
    }

    #[tracing::instrument(skip(self))]
    async fn set_disk_id(&self, id: Option<Uuid>) -> Result<()> {
        let mut lock = self.id.lock().await;
        *lock = id;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn path(&self) -> PathBuf {
        self.root.clone()
    }

    #[tracing::instrument(skip(self))]
    fn get_disk_location(&self) -> DiskLocation {
        DiskLocation {
            pool_idx: {
                if self.endpoint.pool_idx < 0 {
                    None
                } else {
                    Some(self.endpoint.pool_idx as usize)
                }
            },
            set_idx: {
                if self.endpoint.set_idx < 0 {
                    None
                } else {
                    Some(self.endpoint.set_idx as usize)
                }
            },
            disk_idx: {
                if self.endpoint.disk_idx < 0 {
                    None
                } else {
                    Some(self.endpoint.disk_idx as usize)
                }
            },
        }
    }

    #[tracing::instrument(skip(self))]
    async fn make_volume(&self, volume: &str) -> Result<()> {
        info!("make_volume");
        let mut client = node_service_time_out_client(&self.addr)
            .await
            .map_err(|err| DiskError::other(format!("can not get client, err: {err}")))?;
        let request = tonic::Request::new(MakeVolumeRequest {
            disk: self.endpoint.to_string(),
            volume: volume.to_string(),
        });

        let response = client.make_volume(request).await
            .map_err(|e| DiskError::other(format!("gRPC error: {}", e)))?
            .into_inner();

        if !response.success {
            return Err(DiskError::custom(format!("Failed to make volume: {:?}", response.error)));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn make_volumes(&self, volumes: Vec<&str>) -> Result<()> {
        info!("make_volumes");
        let mut client = node_service_time_out_client(&self.addr)
            .await
            .map_err(|err| DiskError::other(format!("can not get client, err: {err}")))?;
        let request = tonic::Request::new(MakeVolumesRequest {
            disk: self.endpoint.to_string(),
            volumes: volumes.iter().map(|s| (*s).to_string()).collect(),
        });

        let response = client.make_volumes(request).await
            .map_err(|e| DiskError::other(format!("gRPC error: {}", e)))?
            .into_inner();

        if !response.success {
            return Err(DiskError::custom(format!("Failed to make volumes: {:?}", response.error)));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn list_volumes(&self) -> Result<Vec<VolumeInfo>> {
        info!("list_volumes");
        let mut client = node_service_time_out_client(&self.addr)
            .await
            .map_err(|err| DiskError::other(format!("can not get client, err: {err}")))?;
        let request = tonic::Request::new(ListVolumesRequest {
            disk: self.endpoint.to_string(),
        });

        let response = client.list_volumes(request).await
            .map_err(|e| DiskError::other(format!("gRPC error: {}", e)))?
            .into_inner();

        if !response.success {
            return Err(DiskError::custom(format!("Failed to list volumes: {:?}", response.error)));
        }

        // Convert protobuf VolumeInfo to our VolumeInfo
        let volumes = response.volume_infos.into_iter().map(|v| VolumeInfo {
            name: v,
            created: None, // TODO: Convert timestamp if available
        }).collect();

        Ok(volumes)
    }

    #[tracing::instrument(skip(self))]
    async fn stat_volume(&self, volume: &str) -> Result<VolumeInfo> {
        info!("stat_volume {}", volume);
        let mut client = node_service_time_out_client(&self.addr)
            .await
            .map_err(|err| DiskError::other(format!("can not get client, err: {err}")))?;
        let request = tonic::Request::new(StatVolumeRequest {
            disk: self.endpoint.to_string(),
            volume: volume.to_string(),
        });

        let response = client.stat_volume(request).await
            .map_err(|e| DiskError::other(format!("gRPC error: {}", e)))?
            .into_inner();

        if !response.success {
            return Err(DiskError::custom(format!("Failed to stat volume: {:?}", response.error)));
        }

        Ok(VolumeInfo {
            name: response.volume_info,
            created: None, // TODO: Add timestamp conversion
        })
    }

    #[tracing::instrument(skip(self))]
    async fn delete_volume(&self, volume: &str) -> Result<()> {
        info!("delete_volume {}", volume);
        let mut client = node_service_time_out_client(&self.addr)
            .await
            .map_err(|err| DiskError::other(format!("can not get client, err: {err}")))?;
        let request = tonic::Request::new(DeleteVolumeRequest {
            disk: self.endpoint.to_string(),
            volume: volume.to_string(),
        });

        let response = client.delete_volume(request).await
            .map_err(|e| DiskError::other(format!("gRPC error: {}", e)))?
            .into_inner();

        if !response.success {
            return Err(DiskError::custom(format!("Failed to delete volume: {:?}", response.error)));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, wr))]
    async fn walk_dir<W: AsyncWrite + Unpin + Send>(&self, opts: WalkDirOptions, wr: &mut W) -> Result<()> {
        info!("walk_dir {}", self.endpoint.to_string());

        let url = format!(
            "{}/rustfs/rpc/walk_dir?disk={}",
            self.endpoint.grid_host(),
            urlencoding::encode(self.endpoint.to_string().as_str()),
        );

        let opts = serde_json::to_vec(&opts)
            .map_err(|e| DiskError::other(format!("JSON serialization error: {}", e)))?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        self.build_auth_headers(&url, &Method::GET, &mut headers);

        let mut reader = HttpReader::new(url, Method::GET, headers, Some(opts)).await
            .map_err(|e| DiskError::other(format!("HTTP error: {}", e)))?;

        tokio::io::copy(&mut reader, wr).await
            .map_err(|e| DiskError::Io(e))?;

        Ok(())
    }

    // For brevity, implement the remaining methods as stubs
    // In a full implementation, these would call the appropriate gRPC methods

    async fn delete_version(
        &self,
        _volume: &str,
        _path: &str,
        _fi: FileInfo,
        _force_del_marker: bool,
        _opts: DeleteOptions,
    ) -> Result<()> {
        Err(DiskError::not_implemented("delete_version"))
    }

    async fn delete_versions(
        &self,
        _volume: &str,
        _versions: Vec<FileInfoVersions>,
        _opts: DeleteOptions,
    ) -> Result<Vec<Option<DiskError>>> {
        Err(DiskError::not_implemented("delete_versions"))
    }

    async fn delete_paths(&self, _volume: &str, _paths: &[String]) -> Result<()> {
        Err(DiskError::not_implemented("delete_paths"))
    }

    async fn write_metadata(&self, _org_volume: &str, _volume: &str, _path: &str, _fi: FileInfo) -> Result<()> {
        Err(DiskError::not_implemented("write_metadata"))
    }

    async fn update_metadata(&self, _volume: &str, _path: &str, _fi: FileInfo, _opts: &UpdateMetadataOpts) -> Result<()> {
        Err(DiskError::not_implemented("update_metadata"))
    }

    async fn read_version(
        &self,
        _org_volume: &str,
        _volume: &str,
        _path: &str,
        _version_id: &str,
        _opts: &ReadOptions,
    ) -> Result<FileInfo> {
        Err(DiskError::not_implemented("read_version"))
    }

    async fn read_xl(&self, _volume: &str, _path: &str, _read_data: bool) -> Result<Vec<u8>> {
        Err(DiskError::not_implemented("read_xl"))
    }

    async fn rename_data(
        &self,
        _src_volume: &str,
        _src_path: &str,
        _fi: FileInfo,
        _dst_volume: &str,
        _dst_path: &str,
    ) -> Result<RenameDataResp> {
        Err(DiskError::not_implemented("rename_data"))
    }

    async fn list_dir(&self, _origvolume: &str, _volume: &str, _dir_path: &str, _count: i32) -> Result<Vec<String>> {
        Err(DiskError::not_implemented("list_dir"))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn read_file(&self, volume: &str, path: &str) -> Result<FileReader> {
        info!("read_file {}/{}", volume, path);

        let url = format!(
            "{}/rustfs/rpc/read_file_stream?disk={}&volume={}&path={}&offset={}&length={}",
            self.endpoint.grid_host(),
            urlencoding::encode(self.endpoint.to_string().as_str()),
            urlencoding::encode(volume),
            urlencoding::encode(path),
            0,
            0
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        self.build_auth_headers(&url, &Method::GET, &mut headers);
        
        let reader = HttpReader::new(url, Method::GET, headers, None).await
            .map_err(|e| DiskError::other(format!("HTTP error: {}", e)))?;
        
        Ok(Box::new(reader))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn read_file_stream(&self, volume: &str, path: &str, offset: usize, length: usize) -> Result<FileReader> {
        let url = format!(
            "{}/rustfs/rpc/read_file_stream?disk={}&volume={}&path={}&offset={}&length={}",
            self.endpoint.grid_host(),
            urlencoding::encode(self.endpoint.to_string().as_str()),
            urlencoding::encode(volume),
            urlencoding::encode(path),
            offset,
            length
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        self.build_auth_headers(&url, &Method::GET, &mut headers);
        
        let reader = HttpReader::new(url, Method::GET, headers, None).await
            .map_err(|e| DiskError::other(format!("HTTP error: {}", e)))?;
        
        Ok(Box::new(reader))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn append_file(&self, volume: &str, path: &str) -> Result<FileWriter> {
        info!("append_file {}/{}", volume, path);

        let url = format!(
            "{}/rustfs/rpc/put_file_stream?disk={}&volume={}&path={}&append={}&size={}",
            self.endpoint.grid_host(),
            urlencoding::encode(self.endpoint.to_string().as_str()),
            urlencoding::encode(volume),
            urlencoding::encode(path),
            true,
            0
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        self.build_auth_headers(&url, &Method::PUT, &mut headers);
        
        let writer = HttpWriter::new(url, Method::PUT, headers).await
            .map_err(|e| DiskError::other(format!("HTTP error: {}", e)))?;
        
        Ok(Box::new(writer))
    }

    async fn create_file(&self, _origvolume: &str, _volume: &str, _path: &str, _file_size: i64) -> Result<FileWriter> {
        Err(DiskError::not_implemented("create_file"))
    }

    async fn rename_file(&self, _src_volume: &str, _src_path: &str, _dst_volume: &str, _dst_path: &str) -> Result<()> {
        Err(DiskError::not_implemented("rename_file"))
    }

    async fn rename_part(&self, _src_volume: &str, _src_path: &str, _dst_volume: &str, _dst_path: &str, _meta: Bytes) -> Result<()> {
        Err(DiskError::not_implemented("rename_part"))
    }

    async fn delete(&self, _volume: &str, _path: &str, _opt: DeleteOptions) -> Result<()> {
        Err(DiskError::not_implemented("delete"))
    }

    async fn verify_file(&self, _volume: &str, _path: &str, _fi: &FileInfo) -> Result<CheckPartsResp> {
        Err(DiskError::not_implemented("verify_file"))
    }

    async fn check_parts(&self, _volume: &str, _path: &str, _fi: &FileInfo) -> Result<CheckPartsResp> {
        Err(DiskError::not_implemented("check_parts"))
    }

    async fn read_multiple(&self, _req: ReadMultipleReq) -> Result<Vec<ReadMultipleResp>> {
        Err(DiskError::not_implemented("read_multiple"))
    }

    async fn write_all(&self, _volume: &str, _path: &str, _data: Bytes) -> Result<()> {
        Err(DiskError::not_implemented("write_all"))
    }

    async fn read_all(&self, _volume: &str, _path: &str) -> Result<Bytes> {
        Err(DiskError::not_implemented("read_all"))
    }

    async fn disk_info(&self, _opts: &DiskInfoOptions) -> Result<DiskInfo> {
        Err(DiskError::not_implemented("disk_info"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_remote_disk_creation() {
        let url = url::Url::parse("http://example.com:9000/path").unwrap();
        let endpoint = Endpoint {
            url: url.clone(),
            is_local: false,
            pool_idx: 0,
            set_idx: 1,
            disk_idx: 2,
        };

        let disk_option = DiskOption {
            cleanup: false,
            health_check: false,
        };

        let remote_disk = RemoteDisk::new(&endpoint, &disk_option).await.unwrap();

        assert!(!remote_disk.is_local());
        assert_eq!(remote_disk.endpoint.url, url);
        assert_eq!(remote_disk.endpoint.pool_idx, 0);
        assert_eq!(remote_disk.endpoint.set_idx, 1);
        assert_eq!(remote_disk.endpoint.disk_idx, 2);
        assert_eq!(remote_disk.host_name(), "example.com:9000");
    }

    #[tokio::test]
    async fn test_remote_disk_basic_properties() {
        let url = url::Url::parse("http://remote-server:9000").unwrap();
        let endpoint = Endpoint {
            url: url.clone(),
            is_local: false,
            pool_idx: -1,
            set_idx: -1,
            disk_idx: -1,
        };

        let disk_option = DiskOption {
            cleanup: false,
            health_check: false,
        };

        let remote_disk = RemoteDisk::new(&endpoint, &disk_option).await.unwrap();

        // Test basic properties
        assert!(!remote_disk.is_local());
        assert_eq!(remote_disk.host_name(), "remote-server:9000");
        assert!(remote_disk.to_string().contains("remote-server"));
        assert!(remote_disk.to_string().contains("9000"));

        // Test disk location
        let location = remote_disk.get_disk_location();
        assert_eq!(location.pool_idx, None);
        assert_eq!(location.set_idx, None);
        assert_eq!(location.disk_idx, None);
        assert!(!location.valid()); // None values make it invalid
    }

    #[tokio::test]
    async fn test_remote_disk_disk_id() {
        let url = url::Url::parse("http://remote-server:9000").unwrap();
        let endpoint = Endpoint {
            url: url.clone(),
            is_local: false,
            pool_idx: 0,
            set_idx: 0,
            disk_idx: 0,
        };

        let disk_option = DiskOption {
            cleanup: false,
            health_check: false,
        };

        let remote_disk = RemoteDisk::new(&endpoint, &disk_option).await.unwrap();

        // Initially, disk ID should be None
        let initial_id = remote_disk.get_disk_id().await.unwrap();
        assert!(initial_id.is_none());

        // Set a disk ID
        let test_id = Uuid::new_v4();
        remote_disk.set_disk_id(Some(test_id)).await.unwrap();

        // Verify the disk ID was set
        let retrieved_id = remote_disk.get_disk_id().await.unwrap();
        assert_eq!(retrieved_id, Some(test_id));

        // Clear the disk ID
        remote_disk.set_disk_id(None).await.unwrap();
        let cleared_id = remote_disk.get_disk_id().await.unwrap();
        assert!(cleared_id.is_none());
    }

    #[tokio::test]
    async fn test_remote_disk_close() {
        let url = url::Url::parse("http://server:9000").unwrap();
        let endpoint = Endpoint {
            url: url.clone(),
            is_local: false,
            pool_idx: 0,
            set_idx: 0,
            disk_idx: 0,
        };

        let disk_option = DiskOption {
            cleanup: false,
            health_check: false,
        };

        let remote_disk = RemoteDisk::new(&endpoint, &disk_option).await.unwrap();

        // Test close operation (should succeed)
        let result = remote_disk.close().await;
        assert!(result.is_ok());
    }
} 