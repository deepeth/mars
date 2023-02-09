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

use std::env;

use common_configs::AzblobStorageConfig;
use common_configs::EthConfig;
use common_configs::FsStorageConfig;
use common_configs::S3StorageConfig;
use common_configs::StorageType;
use common_exceptions::Result;
use opendal::services::Azblob;
use opendal::services::Fs;
use opendal::services::S3;
use opendal::Builder;
use opendal::Operator;

/// init object storage
pub async fn init_object_storage(conf: &EthConfig) -> Result<Operator> {
    match &conf.storage.storage_type {
        StorageType::Fs => init_fs_storage(&conf.storage.fs).await,
        StorageType::S3 => init_s3_operator(&conf.storage.s3).await,
        StorageType::Azure => init_azblob_operator(&conf.storage.azblob).await,
    }
}

/// init_fs_operator will init a opendal fs operator.
async fn init_fs_storage(cfg: &FsStorageConfig) -> Result<Operator> {
    let mut builder = Fs::default();

    let mut path = cfg.data_path.to_string();
    if !path.starts_with('/') {
        path = env::current_dir().unwrap().join(path).display().to_string();
    }
    builder.root(&path);
    Ok(Operator::new(builder.build()?).finish())
}

/// init_s3_operator will init a opendal s3 operator with input s3 config.
async fn init_s3_operator(cfg: &S3StorageConfig) -> Result<Operator> {
    let mut builder = S3::default();

    // Endpoint.
    builder.endpoint(&cfg.endpoint_url);

    // Region
    builder.region(&cfg.region);

    // Credential.
    builder.access_key_id(&cfg.access_key_id);
    builder.secret_access_key(&cfg.secret_access_key);

    // Bucket.
    builder.bucket(&cfg.bucket);

    // Root.
    builder.root(&cfg.root);

    if cfg.enable_virtual_host_style {
        builder.enable_virtual_host_style();
    }
    Ok(Operator::new(builder.build()?).finish())
}

/// init_azblob_operator will init an opendal azblob operator.
async fn init_azblob_operator(cfg: &AzblobStorageConfig) -> Result<Operator> {
    let mut builder = Azblob::default();

    // Endpoint
    builder.endpoint(&cfg.azblob_endpoint_url);

    // Container
    builder.container(&cfg.container);

    // Root
    builder.root(&cfg.azblob_root);

    // Credential
    builder.account_name(&cfg.account_name);
    builder.account_key(&cfg.account_key);

    Ok(Operator::new(builder.build()?).finish())
}
