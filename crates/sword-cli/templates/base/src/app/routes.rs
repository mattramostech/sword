use axum::{routing::get, routing::post, Router};
use sword_ai::FrameworkContext;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::app::controllers::users_controller;
use crate::domain::services::user_service::UserService;
use crate::domain::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService<UserRepository>>,
}

pub fn build_router(ctx: &FrameworkContext) -> Router {
    let user_repository = UserRepository::new(ctx.db.clone());
    let user_service = UserService::new(user_repository);

    let state = AppState {
        user_service: Arc::new(user_service),
    };

    Router::new()
        .route("/users", post(users_controller::create_user))
        .route("/users/:id", get(users_controller::get_user))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
