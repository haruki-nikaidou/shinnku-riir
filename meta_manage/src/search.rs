use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;
use meilisearch_sdk::DefaultHttpClient;
use meilisearch_sdk::documents::DocumentQuery;
use crate::SearchConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ## Games in meilisearch
///
/// use meilisearch to search games. User download game by request the UUID.
pub struct GameMeta {
    pub id: Uuid,
    pub name: String,
    pub catalog: String,
    pub tags: Vec<String>,
    /// Index in VNDB
    pub database_id: Option<i32>,
}

/// For security, manager doesn't support client search.
///
/// You can only
///
/// - find game by exactly id
/// - edit through id
/// - delete through id
/// - add new game
///
/// To query games, reverse proxy meilisearch and use the client search with client key.
pub struct SearchEngineManager(meilisearch_sdk::indexes::Index<DefaultHttpClient>);

const GAME_INDEX: &str = "shinnku-galgame";

impl SearchEngineManager {
    pub async fn new(config: &SearchConfig) -> Result<Self> {
        let client = meilisearch_sdk::client::Client::new(&config.host, Some(&config.api_key))?;
        Ok(Self(client.index(GAME_INDEX)))
    }

    pub async fn add_or_replace(&self, game: &[GameMeta]) -> Result<()> {
        self.0
            .add_or_replace(game, Some("id"))
            .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<GameMeta> {
        let query = DocumentQuery::new(&self.0)
            .execute::<GameMeta>(&id.to_string())
            .await?;
        Ok(query)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        self.0
            .delete_document(&id.to_string())
            .await?;
        Ok(())
    }
}