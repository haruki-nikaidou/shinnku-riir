use crate::config::Account;
use crate::file_list::{DownloadLinkCache, EqualFileDownloadSource, FileDownloadSources};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use redis::aio::MultiplexedConnection;
use redis::Cmd;
use uuid::Uuid;

pub mod onedrive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum FileDownloadProviderName {
    OneDrive
}

enum FileDownloadProvider {
    OneDrive(onedrive::OnedriveDriver)
}

// ==============================================================================================

#[async_trait]
pub trait DownloadProviderStateTrait<T: Sized + Send + Sync>: Sized + Send + Sync {
    async fn get_download_link(&self, source_record: &EqualFileDownloadSource) -> Result<String>;
    async fn from_config(config: T) -> Result<Self>;
    async fn update_self(&self);
}

#[async_trait]
pub trait GetDownloadLinkTrait {
    async fn get_download_link(&self, uuid: Uuid) -> Result<String>;
}

#[async_trait]
pub trait LinkCachedProvider: Sized + Send + Sync {
    fn is_expired_time(create_time: NaiveDateTime) -> bool;
    async fn update_cache(
        &self,
        uuid: Uuid,
        mut source: EqualFileDownloadSource,
        cache: &DownloadLinkCache,
        redis: &MultiplexedConnection
    ) -> Result<()> {
        let key = FileDownloadSources::redis_key(uuid);
        let mut redis = redis.clone();
        let old_source: String = Cmd::get(key.clone()).query_async(&mut redis).await?;
        let mut old_value: Vec<EqualFileDownloadSource> = serde_json::from_str(&old_source)?;
        source.cache_id = Some(cache.0);
        FileDownloadSources::update_item(&mut old_value, source);
        let new_source = serde_json::to_string(&old_value)?;
        redis.send_packed_command(
            &Cmd::set(cache.0.to_string(), serde_json::to_string(&cache.1)?)
        ).await?;
        redis.send_packed_command(
            &Cmd::set(key, new_source)
        ).await?;
        Ok(())
    }
    async fn read_from_cache(
        &self,
        cache_id: Uuid,
        redis: &MultiplexedConnection
    ) -> Result<Option<String>> {
        let key = cache_id.to_string();
        let value: Option<String> = Cmd::get(key).query_async(&mut redis.clone()).await?;
        Ok(value)
    }
    async fn create_link_cache(&self, source_record: &FileDownloadSources) -> Result<DownloadLinkCache>;
}

// ==============================================================================================


// ==============================================================================================

pub struct DownloadProviderManager {
    providers: HashMap<FileDownloadProviderName, FileDownloadProvider>,
    redis: Arc<MultiplexedConnection>
}

impl DownloadProviderManager {
    pub async fn query_by_uuid(
        &self,
        uuid: Uuid,
    ) -> Result<Option<EqualFileDownloadSource>> {
        FileDownloadSources::query_random(uuid, &self.redis).await
    }
}

pub struct DownloadProviderConfig {
    pub account: Vec<Account>,
    pub redis: String
}

#[async_trait]
impl DownloadProviderStateTrait<DownloadProviderConfig> for DownloadProviderManager {
    async fn get_download_link(&self, source_record: &EqualFileDownloadSource) -> Result<String> {
        match (source_record.provider, self.providers.get(&source_record.provider)) {
            (FileDownloadProviderName::OneDrive, Some(FileDownloadProvider::OneDrive(provider))) => {
                provider.get_download_link(source_record).await
            }
            _ => Err(anyhow::anyhow!("No such provider"))
        }
    }

    async fn from_config(config: DownloadProviderConfig) -> Result<Self> {
        let mut onedrive_providers = Vec::new();
        config.account.into_iter().for_each(|account| {
            match account {
                Account::Onedrive(config) => {
                    onedrive_providers.push(config);
                }
            }
        });
        let od_driver = if !onedrive_providers.is_empty() {
            Some(onedrive::OnedriveDriver::from_config(onedrive_providers).await?)
        } else {
            None
        };
        let mut providers = HashMap::new();
        if let Some(od_driver) = od_driver {
            providers.insert(FileDownloadProviderName::OneDrive, FileDownloadProvider::OneDrive(od_driver));
        }
        let redis_client = redis::Client::open(config.redis)?;
        let redis = Arc::new(redis_client.get_multiplexed_tokio_connection().await?);
        Ok(Self { providers, redis })
    }

    async fn update_self(&self) {
        for provider in self.providers.values() {
            match provider {
                FileDownloadProvider::OneDrive(provider) => {
                    provider.update_self().await;
                }
            }
        }
    }
}