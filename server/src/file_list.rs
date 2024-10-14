use anyhow::Result;
use chrono::NaiveDateTime;
use rand::seq::SliceRandom;
use redis::aio::MultiplexedConnection;
use redis::Cmd;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::drivers::FileDownloadProviderName;

const FILE_PATH_HASH_SALT: &str = "shinnku";

pub fn path_to_uuid(path: &str) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_DNS, (path.to_owned() + FILE_PATH_HASH_SALT).as_bytes())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualFileDownloadSource {
    pub provider: FileDownloadProviderName,
    pub token: String,
    pub cache_id: Option<Uuid>,
}

impl PartialEq for EqualFileDownloadSource {
    fn eq(&self, other: &Self) -> bool {
        self.provider == other.provider && self.token == other.token
    }
}

pub struct FileDownloadSources(pub Uuid, pub Vec<EqualFileDownloadSource>);

impl FileDownloadSources {
    pub(crate) fn redis_key(uuid: Uuid) -> String {
        format!("file_source:{}", uuid)
    }

    pub async fn query_random(uuid: Uuid, redis: &MultiplexedConnection) ->
    Result<Option<EqualFileDownloadSource>> {
        let key = FileDownloadSources::redis_key(uuid);
        let value: String = Cmd::get(key).query_async(&mut redis.clone()).await?;
        let value: Vec<EqualFileDownloadSource> = serde_json::from_str(&value)?;
        match value.choose(&mut rand::thread_rng()) {
            Some(v) => Ok(Some(v.clone())),
            None => Ok(None)
        }
    }

    pub async fn set_value(self, redis: &MultiplexedConnection) -> Result<()> {
        let key = FileDownloadSources::redis_key(self.0);
        let value = serde_json::to_string(&self.1)?;
        redis.clone().send_packed_command(
            &Cmd::set(key, value)
        ).await?;
        Ok(())
    }

    pub fn update_item<T: PartialEq + Sized>(list: &mut Vec<T>, item: T) {
        let index = list.iter().position(|x| *x == item);
        if let Some(i) = index {
            list.remove(i);
            list.push(item);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCache {
    pub link: String,
    pub created_at: NaiveDateTime,
}

pub struct DownloadLinkCache(pub Uuid, pub LinkCache);

impl DownloadLinkCache {
    pub(crate) fn redis_key(uuid: Uuid) -> String {
        format!("link_cache:{}", uuid)
    }

    pub async fn create_cache(link: LinkCache, redis: &MultiplexedConnection, ttl: i64) -> Result<Uuid> {
        let uuid = path_to_uuid(&link.link);
        let key = DownloadLinkCache::redis_key(uuid);
        let value = serde_json::to_string(&link)?;
        redis.clone().send_packed_command(
            &Cmd::set(key, value).arg("EX").arg(ttl)
        ).await?;
        Ok(uuid)
    }

    pub async fn read_cache(uuid: Uuid, redis: &MultiplexedConnection) -> Result<Option<LinkCache>> {
        let key = DownloadLinkCache::redis_key(uuid);
        let value: String = Cmd::get(key).query_async(&mut redis.clone()).await?;
        match value.is_empty() {
            true => Ok(None),
            false => {
                let value: LinkCache = serde_json::from_str(&value)?;
                Ok(Some(value))
            }
        }
    }
}