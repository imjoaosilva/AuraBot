use crate::bot::utils;

use super::models::{goal::Goal, repository::Repository};
use sqlx::query;

impl Repository {
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

    pub async fn get_meta(&self) -> Result<i64, sqlx::Error> {
        let result = query!("SELECT goal FROM current_goal")
            .fetch_one(&self.db)
            .await?;

        Ok(result.goal)
    }

    pub async fn get_approved_metas_from_current_week(&self) -> Result<Vec<Goal>, sqlx::Error> {
        let start = utils::get_last_monday_at_18()
            .with_timezone(&chrono_tz::America::Sao_Paulo) 
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let end = utils::get_next_monday_at_18()
            .with_timezone(&chrono_tz::America::Sao_Paulo)
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
