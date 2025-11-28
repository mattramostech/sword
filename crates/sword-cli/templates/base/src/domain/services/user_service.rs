use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepositoryTrait;

pub struct UserService<R: UserRepositoryTrait> {
    repository: R,
}

impl<R: UserRepositoryTrait> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, name: String, email: String) -> anyhow::Result<User> {
        self.repository.create(name, email).await
    }

    pub async fn get_user(&self, id: i64) -> anyhow::Result<Option<User>> {
        self.repository.find_by_id(id).await
    }
}
