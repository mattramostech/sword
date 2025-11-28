use crate::domain::entities::user::User;
use crate::infrastructure::database::models::users;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(&self, name: String, email: String) -> anyhow::Result<User>;
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<User>>;
}

pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, name: String, email: String) -> anyhow::Result<User> {
        let user = users::ActiveModel {
            name: Set(name),
            email: Set(email),
            ..Default::default()
        };

        let result = user.insert(&self.db).await?;

        Ok(User {
            id: result.id,
            created_at: result.created_at,
            name: result.name,
            email: result.email,
        })
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<User>> {
        let user = users::Entity::find_by_id(id).one(&self.db).await?;

        Ok(user.map(|u| User {
            id: u.id,
            created_at: u.created_at,
            name: u.name,
            email: u.email,
        }))
    }
}
