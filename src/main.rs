mod runner;
mod server;

use rmcp::{ServiceExt, transport::stdio};
use runner::TestRunner;
use server::TestRunnerServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse().unwrap())).init();
    let runner = Arc::new(TestRunner::new());
    let server = TestRunnerServer { runner };
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
