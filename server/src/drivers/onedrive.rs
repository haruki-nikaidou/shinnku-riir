use std::collections::HashMap;
use std::marker::{PhantomData, PhantomPinned};
use std::sync::{Arc};
use tokio::sync::RwLock;
use serde::Deserialize;
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use redis::aio::MultiplexedConnection;
use tokio::task::JoinSet;
use tracing::error;
use uuid::Uuid;
use crate::drivers::{DownloadProviderStateTrait, LinkCachedProvider};
use crate::file_list::{DownloadLinkCache, EqualFileDownloadSource, FileDownloadSources};

#[derive(Debug, Deserialize, Clone)]
pub struct OnedriveConfig {
    /// The refresh token for the onedrive account.
    /// *For further information, please refer to the official documentation of Microsoft OAuth 2.0 authorization flow.*
    pub refresh_token: String,

    /// The client id for the application.
    /// You can get it from the Azure portal with the client secret.
    pub client_id: String,

    /// The client secret for the application.
    /// You can get it from the Azure portal with the client id.
    pub client_secret: String,
}

struct OnedriveState {
    pub config: OnedriveConfig,
    pub access_token: Arc<RwLock<String>>,
    pub expires_at: Arc<RwLock<i64>>,
    pub my_drive_id: String,
}

impl OnedriveState {
    pub async fn from_config(config: OnedriveConfig) -> Result<Self> {
        let access_token = fetch_access_token(&config).await?.access_token;
        let expires_at = fetch_access_token(&config).await?.expires_in;
        let my_drive_id = get_my_od_id(&access_token).await?;
        Ok(Self {
            config,
            access_token: Arc::new(RwLock::new(access_token)),
            expires_at: Arc::new(RwLock::new(expires_at)),
            my_drive_id,
        })
    }

    pub async fn update_self(&self) -> Result<()> {
        let res = fetch_access_token(&self.config).await?;
        let mut access_token = self.access_token.write().await;
        let mut expires_at = self.expires_at.write().await;
        *access_token = res.access_token;
        *expires_at = res.expires_in;
        Ok(())
    }
}

// auth
const AUTH_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

#[allow(unused_variables)]
#[derive(Debug, Deserialize)]
/// The response json when request `AUTH_URL`.
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: String,
}

async fn fetch_access_token(config: &OnedriveConfig) -> Result<AccessTokenResponse> {
    let res = reqwest::Client::new()
        .post(AUTH_URL)
        .form(&[
            ("client_id", &config.client_id),
            ("refresh_token", &config.refresh_token),
            ("requested_token_use", &"on_behalf_of".to_owned()),
            ("client_secret", &config.client_secret),
            ("grant_type", &"refresh_token".to_owned()),
        ])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;
    Ok(res)
}

// get my drive id
const MY_DRIVE_URL: &str = "https://graph.microsoft.com/v1.0/me/drive";
async fn get_my_od_id(access_token: &str) -> Result<String> {
    #[derive(Debug, Deserialize)]
    /// Response json when request `MY_DRIVE_URL`.
    struct MyDrive {
        id: String,
    }
    let client = reqwest::Client::new();
    let res = client
        .get(MY_DRIVE_URL)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<MyDrive>()
        .await?;
    Ok(res.id)
}

pub struct OnedriveDriver(HashMap<String, Arc<OnedriveState>>);

fn create_source_token(file_id: String, drive_id: String) -> String {
    format!("{}=:={}", drive_id, file_id)
}
fn parse_source_token(token: &str) -> (String, String) {
    let mut iter = token.split("=:=");
    (iter.next().unwrap().to_owned(), iter.next().unwrap().to_owned())
}

#[async_trait]
impl LinkCachedProvider for OnedriveDriver {
    fn is_expired_time(create_time: NaiveDateTime) -> bool {
        let now = chrono::Utc::now().naive_utc();
        now > create_time + chrono::Duration::minutes(10)
    }
    async fn create_link_cache(&self, source_record: &FileDownloadSources) -> Result<DownloadLinkCache> {
        todo!()
    }
}

#[async_trait]
impl DownloadProviderStateTrait<Vec<OnedriveConfig>> for OnedriveDriver {
    async fn get_download_link(&self, source_record: &EqualFileDownloadSource) -> Result<String> {
        todo!()
    }

    async fn from_config(configs: Vec<OnedriveConfig>) -> Result<Self> {
        let mut map = HashMap::new();
        let mut futures = JoinSet::new();
        for config in configs {
            futures.spawn(OnedriveState::from_config(config));
        }
        while let Some(res) = futures.join_next().await {
            let state = res??;
            map.insert(state.my_drive_id.clone(), Arc::new(state));
        }
        Ok(Self(map))
    }

    async fn update_self(&self) {
        let mut futures = JoinSet::new();
        let states: Vec<Arc<OnedriveState>> = self.0
            .iter().map(|(_, state)| Arc::clone(state)).collect();
        for state in states {
            futures.spawn(async move {
                state.update_self().await
            });
        }
        while let Some(res) = futures.join_next().await {
            match res {
                Ok(Err(_)) => {
                    error!("Failed to update onedrive state");
                }
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to update onedrive state: {:?}", e);
                }
            }
        }
    }
}

