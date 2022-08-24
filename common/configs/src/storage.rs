// Copyright 2022 BohuTANG.
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
// Copy from https://github.com/Sherlock-Holo/ddns/blob/master/src/trace.rs

use std::fmt;

use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    #[clap(long, default_value = "fs")]
    #[serde(rename = "type", alias = "storage_type")]
    pub storage_type: String,

    // S3 storage backend config.
    #[clap(flatten)]
    pub s3: S3StorageConfig,

    // azure storage blob config.
    #[clap(flatten)]
    pub azblob: AzblobStorageConfig,
}

impl Default for StorageConfig {
    fn default() -> Self {
        StorageConfig {
            storage_type: "fs".to_string(),
            s3: Default::default(),
            azblob: Default::default(),
        }
    }
}

#[derive(Parser, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct S3StorageConfig {
    /// Region for S3 storage
    #[clap(long = "storage-s3-region", default_value_t)]
    pub region: String,

    /// Endpoint URL for S3 storage
    #[clap(
        long = "storage-s3-endpoint-url",
        default_value = "https://s3.amazonaws.com"
    )]
    pub endpoint_url: String,

    /// Access key for S3 storage
    #[clap(long = "storage-s3-access-key-id", default_value_t)]
    pub access_key_id: String,

    /// Secret key for S3 storage
    #[clap(long = "storage-s3-secret-access-key", default_value_t)]
    pub secret_access_key: String,

    /// S3 Bucket to use for storage
    #[clap(long = "storage-s3-bucket", default_value_t)]
    pub bucket: String,

    /// <root>
    #[clap(long = "storage-s3-root", default_value_t)]
    pub root: String,
}

impl Default for S3StorageConfig {
    fn default() -> Self {
        S3StorageConfig {
            region: "".to_string(),
            endpoint_url: "https://s3.amazonaws.com".to_string(),
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            bucket: "".to_string(),
            root: "".to_string(),
        }
    }
}

impl fmt::Debug for S3StorageConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("S3StorageConfig")
            .field("endpoint_url", &self.endpoint_url)
            .field("region", &self.region)
            .field("bucket", &self.bucket)
            .field("root", &self.root)
            .field("access_key_id", &mask_string(&self.access_key_id, 3))
            .field(
                "secret_access_key",
                &mask_string(&self.secret_access_key, 3),
            )
            .finish()
    }
}

#[derive(Parser, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AzblobStorageConfig {
    /// Account for Azblob
    #[clap(long = "storage-azblob-account-name", default_value_t)]
    pub account_name: String,

    /// Master key for Azblob
    #[clap(long = "storage-azblob-account-key", default_value_t)]
    pub account_key: String,

    /// Container for Azblob
    #[clap(long = "storage-azblob-container", default_value_t)]
    pub container: String,

    /// Endpoint URL for Azblob
    /// Clap doesn't allow us to use endpoint_url directly.
    #[clap(long = "storage-azblob-endpoint-url", default_value_t)]
    pub azblob_endpoint_url: String,

    /// Clap doesn't allow us to use root directly.
    #[clap(long = "storage-azblob-root", default_value_t)]
    pub azblob_root: String,
}

impl Default for AzblobStorageConfig {
    fn default() -> Self {
        AzblobStorageConfig {
            account_name: "".to_string(),
            account_key: "".to_string(),
            container: "".to_string(),
            azblob_endpoint_url: "".to_string(),
            azblob_root: "".to_string(),
        }
    }
}

impl fmt::Debug for AzblobStorageConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AzureStorageBlobConfig")
            .field("endpoint_url", &self.azblob_endpoint_url)
            .field("container", &self.container)
            .field("root", &self.azblob_root)
            .field("account_name", &mask_string(&self.account_name, 3))
            .field("account_key", &mask_string(&self.account_key, 3))
            .finish()
    }
}

/// Mask a string by "******", but keep `unmask_len` of suffix.
fn mask_string(s: &str, unmask_len: usize) -> String {
    if s.len() <= unmask_len {
        s.to_string()
    } else {
        let mut ret = "******".to_string();
        ret.push_str(&s[(s.len() - unmask_len)..]);
        ret
    }
}
