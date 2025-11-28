mod app;
mod domain;
mod infrastructure;

use sword_ai::server;
use infrastructure::database::migration::Migrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    sword_ai::tracing::init_tracing();

    server::run_with_migrator::<Migrator, _>(
        |ctx| app::routes::build_router(ctx),
        true,
    )
    .await
}
