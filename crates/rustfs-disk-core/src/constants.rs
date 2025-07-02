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

//! Constants used throughout the disk subsystem

pub const RUSTFS_META_BUCKET: &str = ".rustfs.sys";
pub const RUSTFS_META_MULTIPART_BUCKET: &str = ".rustfs.sys/multipart";
pub const RUSTFS_META_TMP_BUCKET: &str = ".rustfs.sys/tmp";
pub const RUSTFS_META_TMP_DELETED_BUCKET: &str = ".rustfs.sys/tmp/.trash";
pub const BUCKET_META_PREFIX: &str = "buckets";
pub const FORMAT_CONFIG_FILE: &str = "format.json";
pub const STORAGE_FORMAT_FILE: &str = "xl.meta";
pub const STORAGE_FORMAT_FILE_BACKUP: &str = "xl.meta.bkp";

// Check part status constants
pub const CHECK_PART_UNKNOWN: usize = 0;
pub const CHECK_PART_SUCCESS: usize = 1;
pub const CHECK_PART_DISK_NOT_FOUND: usize = 2;
pub const CHECK_PART_VOLUME_NOT_FOUND: usize = 3;
pub const CHECK_PART_FILE_NOT_FOUND: usize = 4;
pub const CHECK_PART_FILE_CORRUPT: usize = 5;
