use super::models::repository::Repository;
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
}
