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

//! # RustFS Disk Core
//!
//! This crate provides the core abstractions and traits for disk operations in RustFS.
//! It defines the fundamental interfaces that different disk implementations must follow.

pub mod constants;
pub mod endpoint;
pub mod error;
pub mod error_conv;
pub mod format;
pub mod traits;
pub mod types;

// Re-export commonly used items
pub use constants::*;
pub use endpoint::*;
pub use error::*;
pub use error_conv::*;
pub use format::*;
pub use traits::*;
pub use types::*;
