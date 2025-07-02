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

//! # RustFS Remote Disk Implementation
//!
//! This crate provides the remote disk implementation for RustFS.
//! It implements the DiskAPI trait for remote disk operations via HTTP/gRPC.

pub mod remote;

// Re-export commonly used items
pub use remote::RemoteDisk;

/// Create a new remote disk instance
pub async fn new_remote_disk(
    ep: &rustfs_disk_core::Endpoint,
    opt: &rustfs_disk_core::DiskOption,
) -> rustfs_disk_core::Result<RemoteDisk> {
    RemoteDisk::new(ep, opt).await
} 