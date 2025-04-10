use super::models::{channels::ChannelIds, repository::Repository};
use sqlx::query;

impl Repository {
    pub async fn get_user_channel(&self, user_id: u64) -> Result<Option<u64>, sqlx::Error> {
        let user_id = user_id.to_string();
        let result = query!("SELECT channel_id FROM users WHERE discord_id = ?", user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.map(|r| r.channel_id.parse::<u64>().ok()).flatten())
    }

    pub async fn create_user_channel(
        &self,
        user_id: u64,
        channel_id: u64,
    ) -> Result<(), sqlx::Error> {
        let user_id = user_id.to_string();
        let channel_id = channel_id.to_string();
        query!(
            "INSERT INTO users (discord_id, channel_id) VALUES (?, ?)",
            user_id,
            channel_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn set_meta(&self, amount: i64) -> Result<(), sqlx::Error> {
        let current = query!("SELECT goal FROM current_goal")
            .fetch_optional(&self.db)
            .await?;

        if let Some(_) = current {
            query!(
                "UPDATE current_goal SET goal = ?, created_at = CURRENT_TIMESTAMP",
                amount
            )
            .execute(&self.db)
            .await?;
        } else {
            query!("INSERT INTO current_goal (goal) VALUES (?)", amount)
                .execute(&self.db)
                .await?;
        }

        Ok(())
    }

    pub async fn set_channels(
        &self,
        individual: u64,
        anonymous: u64,
        meta: u64,
        logs: u64,
    ) -> Result<(), sqlx::Error> {
        let individual = individual as i64;
        let anonymous = anonymous as i64;
        let meta = meta as i64;
        let logs = logs as i64;

        let current = query!("SELECT * FROM channels")
            .fetch_optional(&self.db)
            .await?;

        if let Some(_) = current {
            query!(
                "UPDATE channels SET logs_channel_id = ?, meta_channel_id = ?, anonymous_channel_id = ?, individuals_category_id = ?",
                logs,
                meta,
                anonymous,
                individual
            )
            .execute(&self.db)
            .await?;
        } else {
            query!(
                "INSERT INTO channels (logs_channel_id, meta_channel_id, anonymous_channel_id, individuals_category_id) VALUES (?, ?, ?, ?)",
                logs,
                meta,
                anonymous,
                individual
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
                logs_channel_id 
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
        })
    }
}
