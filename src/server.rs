use crate::runner::TestRunner;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RunUnitTestsInput {
    pub path: String,
    pub filter: Option<String>,
    pub package: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RunIntegrationTestsInput {
    pub path: String,
    pub filter: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RunBrowserTestsInput {
    pub path: String,
    pub filter: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RunTargetedTestsInput {
    pub path: String,
    pub test_names: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetTestResultsInput {
    pub path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetCoverageReportInput {
    pub path: String,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CollectTestLogsInput {
    pub path: String,
    pub filter: Option<String>,
}

#[derive(Clone)]
pub struct TestRunnerServer {
    pub runner: Arc<TestRunner>,
}

#[tool_router(server_handler)]
impl TestRunnerServer {
    #[tool(description = "Run unit tests in a project. Auto-detects test framework (cargo test, pytest, jest, go test).")]
    async fn run_unit_tests(&self, Parameters(i): Parameters<RunUnitTestsInput>) -> String {
        let result = self.runner.run_tests(&i.path, "unit", i.filter.as_deref(), i.package.as_deref()).await;
        serde_json::to_string_pretty(&result).unwrap()
    }

    #[tool(description = "Run integration tests. Looks for tests/ directory or integration test markers.")]
    async fn run_integration_tests(&self, Parameters(i): Parameters<RunIntegrationTestsInput>) -> String {
        let result = self.runner.run_tests(&i.path, "integration", i.filter.as_deref(), None).await;
        serde_json::to_string_pretty(&result).unwrap()
    }

    #[tool(description = "Run browser/E2E tests (Playwright, Cypress, Selenium).")]
    async fn run_browser_tests(&self, Parameters(i): Parameters<RunBrowserTestsInput>) -> String {
        let result = self.runner.run_tests(&i.path, "browser", i.filter.as_deref(), None).await;
        serde_json::to_string_pretty(&result).unwrap()
    }

    #[tool(description = "Run specific tests by name — useful for targeted regression testing.")]
    async fn run_targeted_tests(&self, Parameters(i): Parameters<RunTargetedTestsInput>) -> String {
        let filter = i.test_names.join("|");
        let result = self.runner.run_tests(&i.path, "targeted", Some(&filter), None).await;
        serde_json::to_string_pretty(&result).unwrap()
    }

    #[tool(description = "Get results from the last test run (pass/fail counts, failed test names).")]
    async fn get_test_results(&self, Parameters(i): Parameters<GetTestResultsInput>) -> String {
        let result = self.runner.get_last_results(&i.path);
        serde_json::to_string_pretty(&result).unwrap()
    }

    #[tool(description = "Get test coverage report. Runs coverage tool if available (cargo-tarpaulin, coverage.py, c8).")]
    async fn get_coverage_report(&self, Parameters(i): Parameters<GetCoverageReportInput>) -> String {
        let result = self.runner.get_coverage(&i.path, i.format.as_deref()).await;
        serde_json::to_string_pretty(&result).unwrap()
    }

    #[tool(description = "Collect test output logs, including stdout/stderr from failed tests.")]
    async fn collect_test_logs(&self, Parameters(i): Parameters<CollectTestLogsInput>) -> String {
        let result = self.runner.collect_logs(&i.path, i.filter.as_deref());
        serde_json::to_string_pretty(&result).unwrap()
    }
}
