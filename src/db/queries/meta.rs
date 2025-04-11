use super::models::repository::Repository;
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
}
