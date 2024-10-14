use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotRecord {
    pub game_id: Uuid,
    pub ip: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberedHotRecord {
    pub id: i64,
    #[serde(flatten)]
    pub hot_record: HotRecord,
    pub download_time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCountItem {
    pub game_id: Uuid,
    pub count: i64
}

impl HotRecord {
    pub async fn insert(self, pg: PgPool) -> Result<()> {
        query!(
            r#"
            INSERT INTO hot_static (game_id, ip)
            VALUES ($1, $2)
            "#,
            self.game_id, self.ip
        )
            .execute(&pg)
            .await?;
        Ok(())
    }

    pub async fn get_top10(pg: PgPool) -> Result<Vec<TopCountItem>> {
        let res = query!(
            r#"
            SELECT game_id, count
            FROM top_10_downloaded_games
            "#,
        )
            .fetch_all(&pg)
            .await?
            .into_iter()
            .map(|row| TopCountItem {
                game_id: row.game_id.unwrap(), // safely unwrap game_id
                count: row.count.unwrap_or(0), // safely unwrap count
            })
            .collect();
        Ok(res)
    }
}