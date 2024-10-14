use crate::config::Account;
use crate::file_list::EqualFileDownloadSource;
use anyhow::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

pub mod onedrive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum FileDownloadProviderName {
    OneDrive
}

enum FileDownloadProvider {
    OneDrive(onedrive::OnedriveDriver)
}

#[async_trait]
pub trait DownloadProvider<T: Sized + Send + Sync>: Sized + Send + Sync {
    async fn get_download_link(&self, source_record: &EqualFileDownloadSource) -> Result<String>;
    async fn from_config(config: Vec<T>) -> Result<Self>;
    async fn update_self(&self);
}

pub struct DownloadProviderManager {
    providers: HashMap<FileDownloadProviderName, FileDownloadProvider>
}

#[async_trait]
impl DownloadProvider<Account> for DownloadProviderManager {
    async fn get_download_link(&self, source_record: &EqualFileDownloadSource) -> Result<String> {
        match (source_record.provider, self.providers.get(&source_record.provider)) {
            (FileDownloadProviderName::OneDrive, Some(FileDownloadProvider::OneDrive(provider))) => {
                provider.get_download_link(source_record).await
            }
            _ => Err(anyhow::anyhow!("No such provider"))
        }
    }

    async fn from_config(config: Vec<Account>) -> Result<Self> {
        let mut onedrive_providers = Vec::new();
        config.into_iter().for_each(|account| {
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
        Ok(Self { providers })
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