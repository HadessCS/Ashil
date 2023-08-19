use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

pub fn write_json_report(report_path: &str, vulnerabilities: &[owasp::VulnerabilityResult]) {
    let json_report = serde_json::to_string_pretty(vulnerabilities).unwrap();

    let mut file = File::create(report_path).expect("Failed to create report file");
    file.write_all(json_report.as_bytes()).expect("Failed to write JSON report");
}

