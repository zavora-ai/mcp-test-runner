# Test Runner MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-test-runner.svg)](https://crates.io/crates/mcp-test-runner)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Let your AI agents run tests. This MCP server auto-detects your test framework and runs unit tests, integration tests, browser tests, and coverage — then reports results the agent can act on.

## What It Does

When your agent writes code or fixes a bug, it can immediately verify the fix by running tests. No manual intervention needed.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-test-runner/main/docs/architecture.svg" alt="Test Runner MCP Architecture" width="700"/>
</p>

## Supported Frameworks

The server auto-detects your project type:

| Framework | Detection | Test Command | Coverage |
|-----------|-----------|-------------|----------|
| **Cargo** (Rust) | `Cargo.toml` | `cargo test` | `cargo tarpaulin` |
| **Jest** (JS/TS) | `package.json` | `npx jest` | `c8` |
| **Pytest** (Python) | `pyproject.toml` / `pytest.ini` | `pytest` | `coverage.py` |
| **Go** | `go.mod` | `go test ./...` | `-coverprofile` |
| **Playwright** | `playwright.config.ts` | `npx playwright test` | — |
| **Cypress** | `cypress.config.ts` | `npx cypress run` | — |

## Tools (7)

| Tool | What It Does | When To Use |
|------|-------------|-------------|
| `run_unit_tests` | Runs unit tests with optional filter | "Run the tests" / "Test the auth module" |
| `run_integration_tests` | Runs integration tests (tests/ dir) | "Run integration tests" |
| `run_browser_tests` | Runs E2E tests (Playwright/Cypress) | "Run the browser tests" |
| `run_targeted_tests` | Runs specific tests by name | "Run test_login and test_signup" |
| `get_test_results` | Shows last run results | "Did the tests pass?" |
| `get_coverage_report` | Runs coverage analysis | "What's the test coverage?" |
| `collect_test_logs` | Gets stdout/stderr from test runs | "Show me the test output" |

## Verified Output

Tested against a real Rust project:

```
> run_unit_tests(path: "/my-project")

{
  "framework": "cargo",
  "kind": "unit",
  "passed": true,
  "pass_count": 12,
  "fail_count": 0,
  "exit_code": 0,
  "output": "running 12 tests\ntest test_create ... ok\ntest test_update ... ok\n...\ntest result: ok. 12 passed; 0 failed; 0 ignored"
}

> run_targeted_tests(path: "/my-project", test_names: ["test_auth", "test_login"])

{
  "framework": "cargo",
  "kind": "targeted",
  "passed": true,
  "pass_count": 2,
  "fail_count": 0
}

> get_test_results(path: "/my-project")

{
  "framework": "cargo",
  "passed": true,
  "pass_count": 2,
  "fail_count": 0
}

> collect_test_logs(path: "/my-project", filter: "FAILED")

{
  "lines": 0,
  "logs": ""
}
```

## Installation

### 1. Build

```bash
git clone https://github.com/zavora-ai/mcp-test-runner
cd mcp-test-runner
cargo build --release
```

### 2. Add to your MCP client

**Claude Desktop:**
```json
{
  "mcpServers": {
    "test-runner": {
      "command": "/path/to/mcp-test-runner"
    }
  }
}
```

**Kiro:**
```json
{
  "mcpServers": {
    "test-runner": {
      "command": "/path/to/mcp-test-runner"
    }
  }
}
```

**Cursor / Windsurf:**
```json
{
  "mcpServers": {
    "test-runner": {
      "command": "/path/to/mcp-test-runner"
    }
  }
}
```

### 3. Use it

Ask your agent:
- "Run the tests"
- "Run only the auth tests"
- "Did the tests pass?"
- "What's the test coverage?"
- "Show me the output from the failed tests"

## How It Works

1. You tell the agent to run tests
2. The agent calls `run_unit_tests` with the project path
3. The server detects the framework (Cargo.toml → cargo test)
4. Tests run locally and output is captured
5. Results are returned with pass/fail counts
6. The agent can then fix failures and re-run

No configuration needed — just point at a directory.

## MCP Server Manifest

```toml
server_id = "mcp_test_runner"
display_name = "Test Runner MCP"
version = "1.0.0"
domain = "developer"
risk_level = "low"
writes_allowed = "none"
transports = ["stdio"]
governance_gates = []
```

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability

