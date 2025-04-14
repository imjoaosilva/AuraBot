use super::models::{channels::ChannelIds, repository::Repository};
use sqlx::query;

impl Repository {
    pub async fn set_channels(
        &self,
        individual: u64,
        anonymous: u64,
        meta: u64,
        logs: u64,
        approval: u64,
        results: u64,
    ) -> Result<(), sqlx::Error> {
        let individual = individual as i64;
        let anonymous = anonymous as i64;
        let meta = meta as i64;
        let logs = logs as i64;
        let approval = approval as i64;
        let results = results as i64;

        let current = query!("SELECT * FROM channels")
            .fetch_optional(&self.db)
            .await?;

        if let Some(_) = current {
            query!(
                "UPDATE channels SET logs_channel_id = ?, meta_channel_id = ?, anonymous_channel_id = ?, individuals_category_id = ?, approval_channel_id = ?, results_channel_id = ?",
                logs,
                meta,
                anonymous,
                individual,
                approval,
                results
            )
            .execute(&self.db)
            .await?;
        } else {
            query!(
                "INSERT INTO channels (logs_channel_id, meta_channel_id, anonymous_channel_id, individuals_category_id, approval_channel_id, results_channel_id) VALUES (?, ?, ?, ?, ?, ?)",
                logs,
                meta,
                anonymous,
                individual,
                approval,
                results
            )
            .execute(&self.db)
            .await?;
        }

        Ok(())
    }

    pub async fn get_channels(&self) -> Result<ChannelIds, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT 
                individuals_category_id, 
                anonymous_channel_id, 
                meta_channel_id, 
                logs_channel_id ,
                approval_channel_id,
                results_channel_id
            FROM channels
            "#
        )
        .fetch_one(&self.db)
        .await?;

        Ok(ChannelIds {
            individuals_category_id: result.individuals_category_id as u64,
            anonymous_channel_id: result.anonymous_channel_id as u64,
            meta_channel_id: result.meta_channel_id as u64,
            logs_channel_id: result.logs_channel_id as u64,
            approval_channel_id: result.approval_channel_id as u64,
            results_channel_id: result.results_channel_id as u64,
        })
    }
}
