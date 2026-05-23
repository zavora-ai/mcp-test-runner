mod runner;
mod server;

use rmcp::{ServiceExt, transport::stdio};
use runner::TestRunner;
use server::TestRunnerServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let runner = Arc::new(TestRunner::new());
    let server = TestRunnerServer { runner };
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
