pub struct Goal {
    pub id: i64,
    pub user_id: i64,
    pub amount: i64,
    pub created_at: Option<String>,
    pub status: Option<String>,
    pub message_id: Option<String>,
}
