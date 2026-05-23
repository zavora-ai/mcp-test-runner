use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

pub struct TestRunner {
    last_results: Mutex<Option<Value>>,
    last_logs: Mutex<String>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self { last_results: Mutex::new(None), last_logs: Mutex::new(String::new()) }
    }

    fn detect_framework(path: &str) -> &'static str {
        if Path::new(&format!("{}/Cargo.toml", path)).exists() { return "cargo"; }
        if Path::new(&format!("{}/package.json", path)).exists() {
            if Path::new(&format!("{}/playwright.config.ts", path)).exists() { return "playwright"; }
            if Path::new(&format!("{}/cypress.config.ts", path)).exists() { return "cypress"; }
            return "jest";
        }
        if Path::new(&format!("{}/pytest.ini", path)).exists() || Path::new(&format!("{}/pyproject.toml", path)).exists() { return "pytest"; }
        if Path::new(&format!("{}/go.mod", path)).exists() { return "go"; }
        "unknown"
    }

    pub async fn run_tests(&self, path: &str, kind: &str, filter: Option<&str>, package: Option<&str>) -> Value {
        let framework = Self::detect_framework(path);
        let (cmd, args) = match (framework, kind) {
            ("cargo", "unit") => {
                let mut a = vec!["test"];
                if let Some(p) = package { a.extend(["--package", p]); }
                if let Some(f) = filter { a.push(f); }
                a.push("--");
                a.push("--nocapture");
                ("cargo", a)
            }
            ("cargo", "integration") => {
                let mut a = vec!["test", "--test", "*"];
                if let Some(f) = filter { a.push(f); }
                ("cargo", a)
            }
            ("cargo", "targeted") => {
                let mut a = vec!["test"];
                if let Some(f) = filter { a.push(f); }
                ("cargo", a)
            }
            ("jest", "unit") | ("jest", "targeted") => {
                let mut a = vec!["test", "--", "--passWithNoTests"];
                if let Some(f) = filter { a.extend(["--testNamePattern", f]); }
                ("npx", a)
            }
            ("jest", "browser") | ("playwright", _) => ("npx", vec!["playwright", "test"]),
            ("pytest", _) => {
                let mut a = vec!["-x", "--tb=short"];
                if let Some(f) = filter { a.extend(["-k", f]); }
                ("pytest", a)
            }
            ("go", _) => {
                let mut a = vec!["test", "./..."];
                if let Some(f) = filter { a.extend(["-run", f]); }
                ("go", a)
            }
            _ => ("echo", vec!["No test framework detected"]),
        };

        let output = Command::new(cmd).args(&args).current_dir(path).output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let combined = format!("{}\n{}", stdout, stderr);
                let passed = out.status.success();

                let (pass_count, fail_count) = Self::parse_counts(&combined, framework);

                let result = json!({
                    "framework": framework,
                    "kind": kind,
                    "passed": passed,
                    "pass_count": pass_count,
                    "fail_count": fail_count,
                    "exit_code": out.status.code(),
                    "output": combined.lines().take(50).collect::<Vec<_>>().join("\n"),
                    "truncated": combined.lines().count() > 50,
                });

                *self.last_results.lock().unwrap() = Some(result.clone());
                *self.last_logs.lock().unwrap() = combined;
                result
            }
            Err(e) => json!({"error": format!("Failed to run {}: {}", cmd, e), "framework": framework}),
        }
    }

    fn parse_counts(output: &str, framework: &str) -> (u64, u64) {
        match framework {
            "cargo" => {
                // "test result: ok. 5 passed; 0 failed;"
                let pass = output.lines().rev().find(|l| l.contains("test result"))
                    .and_then(|l| l.split("passed").next())
                    .and_then(|s| s.split_whitespace().last())
                    .and_then(|n| n.parse().ok()).unwrap_or(0);
                let fail = output.lines().rev().find(|l| l.contains("test result"))
                    .and_then(|l| l.split("failed").next())
                    .and_then(|s| s.split_whitespace().last())
                    .and_then(|n| n.parse().ok()).unwrap_or(0);
                (pass, fail)
            }
            _ => (0, 0),
        }
    }

    pub fn get_last_results(&self, _path: &str) -> Value {
        self.last_results.lock().unwrap().clone().unwrap_or(json!({"message": "No test results available. Run tests first."}))
    }

    pub async fn get_coverage(&self, path: &str, _format: Option<&str>) -> Value {
        let framework = Self::detect_framework(path);
        let (cmd, args): (&str, Vec<&str>) = match framework {
            "cargo" => ("cargo", vec!["tarpaulin", "--out", "json", "--skip-clean"]),
            "jest" => ("npx", vec!["jest", "--coverage", "--coverageReporters=json-summary"]),
            "pytest" => ("pytest", vec!["--cov", "--cov-report=json"]),
            "go" => ("go", vec!["test", "-coverprofile=coverage.out", "./..."]),
            _ => return json!({"error": "No coverage tool detected for this framework"}),
        };

        let output = Command::new(cmd).args(&args).current_dir(path).output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                json!({
                    "framework": framework,
                    "tool": cmd,
                    "success": out.status.success(),
                    "output": format!("{}\n{}", stdout, stderr).lines().take(30).collect::<Vec<_>>().join("\n"),
                })
            }
            Err(e) => json!({"error": format!("Coverage tool not available: {}", e), "hint": "Install cargo-tarpaulin, c8, or coverage.py"}),
        }
    }

    pub fn collect_logs(&self, _path: &str, filter: Option<&str>) -> Value {
        let logs = self.last_logs.lock().unwrap().clone();
        let lines: Vec<&str> = if let Some(f) = filter {
            logs.lines().filter(|l| l.contains(f)).collect()
        } else {
            logs.lines().collect()
        };
        json!({"lines": lines.len(), "logs": lines.into_iter().take(100).collect::<Vec<_>>().join("\n")})
    }
}
