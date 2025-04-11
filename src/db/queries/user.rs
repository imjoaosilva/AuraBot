use crate::bot::utils;

use super::models::{goal::Goal, repository::Repository};
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

    pub async fn get_user_last_goal(&self, user_id: u64) -> Result<Option<Goal>, sqlx::Error> {
        let user_id = user_id.to_string();
        let result = query!(
            "SELECT id, discord_id, amount, created_at, status, message_id FROM goals WHERE discord_id = ? ORDER BY created_at DESC LIMIT 1",
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = result {
            let goal = Goal {
                id: row.id,
                user_id: row.discord_id,
                amount: row.amount,
                created_at: row.created_at,
                status: row.status,
                message_id: row.message_id,
            };
            Ok(Some(goal))
        } else {
            Ok(None)
        }
    }

    pub async fn create_goal(
        &self,
        user_id: u64,
        amount: i64,
        message_id: String,
    ) -> Result<(), sqlx::Error> {
        let user_id = user_id.to_string();
        query!(
            "INSERT INTO goals (discord_id, amount, message_id) VALUES (?, ?, ?)",
            user_id,
            amount,
            message_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_user_meta_by_message_id(&self, message_id: u64) -> Result<Goal, sqlx::Error> {
        let message_id = message_id.to_string();

        let result = query!("SELECT * FROM goals WHERE message_id = ?", message_id)
            .fetch_one(&self.db)
            .await?;

        let goal = Goal {
            id: result.id,
            user_id: result.discord_id,
            amount: result.amount,
            message_id: result.message_id,
            created_at: result.created_at,
            status: result.status,
        };

        Ok(goal)
    }

    pub async fn update_meta_status(
        &self,
        message_id: u64,
        status: String,
    ) -> Result<(), sqlx::Error> {
        let message_id = message_id.to_string();
        query!(
            "UPDATE goals SET status = ? WHERE message_id = ?",
            status,
            message_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_approved_metas_from_current_week(&self) -> Result<Vec<Goal>, sqlx::Error> {
        let start = utils::get_last_monday_at_18()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let end = utils::get_next_monday_at_18()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let rows = sqlx::query!(
            r#"
            SELECT id, discord_id, amount, created_at, status, message_id
            FROM goals
            WHERE status = 'Approved'
              AND datetime(created_at) >= datetime(?)
              AND datetime(created_at) < datetime(?)
            "#,
            start,
            end
        )
        .fetch_all(&self.db)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Goal {
                id: row.id,
                user_id: row.discord_id,
                amount: row.amount,
                created_at: row.created_at,
                status: row.status,
                message_id: row.message_id,
            })
            .collect())
    }
}
