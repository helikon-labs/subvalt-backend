use crate::postgres::network::PostgreSQLNetworkStorage;
use subvt_types::app::event::democracy::DemocracyCancelledEvent;

impl PostgreSQLNetworkStorage {
    pub async fn save_democracy_cancelled_event(
        &self,
        block_hash: &str,
        extrinsic_index: Option<i32>,
        event_index: i32,
        referendum_index: u32,
    ) -> anyhow::Result<Option<i32>> {
        let maybe_result: Option<(i32,)> = sqlx::query_as(
            r#"
            INSERT INTO sub_event_democracy_cancelled (block_hash, extrinsic_index, event_index, referendum_index)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
            .bind(block_hash)
            .bind(extrinsic_index)
            .bind(event_index)
            .bind(referendum_index as i64)
            .fetch_optional(&self.connection_pool)
            .await?;
        if let Some(result) = maybe_result {
            Ok(Some(result.0))
        } else {
            Ok(None)
        }
    }

    pub async fn get_democracy_cancelled_events_in_block(
        &self,
        block_hash: &str,
    ) -> anyhow::Result<Vec<DemocracyCancelledEvent>> {
        let db_events: Vec<(i32, String, Option<i32>, i32, i64)> = sqlx::query_as(
            r#"
            SELECT "id", block_hash, extrinsic_index, event_index, referendum_index
            FROM sub_event_democracy_cancelled
            WHERE block_hash = $1
            ORDER BY "id" ASC
            "#,
        )
        .bind(block_hash)
        .fetch_all(&self.connection_pool)
        .await?;
        let mut events = Vec::new();
        for db_event in db_events {
            events.push(DemocracyCancelledEvent {
                id: db_event.0 as u32,
                block_hash: db_event.1.clone(),
                extrinsic_index: db_event.2.map(|index| index as u32),
                event_index: db_event.3 as u32,
                referendum_index: db_event.4 as u64,
            })
        }
        Ok(events)
    }
}
