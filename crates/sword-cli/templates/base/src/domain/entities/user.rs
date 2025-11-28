use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
}
