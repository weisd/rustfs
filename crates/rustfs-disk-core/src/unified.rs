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

//! Unified disk implementation that combines local and remote disks

use crate::{DiskOption, Endpoint, Result};

/// Factory function to create a new disk instance based on endpoint
/// This function will be implemented when we integrate with the actual disk implementations
pub async fn new_disk(ep: &Endpoint, opt: &DiskOption) -> Result<()> {
    if ep.is_local {
        // This will be implemented when we integrate with rustfs-disk-local
        Err(crate::DiskError::not_implemented("Local disk integration pending"))
    } else {
        // This will be implemented when we integrate with rustfs-disk-remote
        Err(crate::DiskError::not_implemented("Remote disk integration pending"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_disk_placeholder() {
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

        // For now, this should return a not implemented error
        let result = new_disk(&endpoint, &disk_option).await;
        assert!(result.is_err());

        if let Err(crate::DiskError::NotImplemented { operation }) = result {
            assert!(operation.contains("Remote disk integration pending"));
        } else {
            panic!("Expected NotImplemented error");
        }
    }
}
