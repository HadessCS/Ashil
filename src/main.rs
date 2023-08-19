#![allow(warnings)]

mod scanner;
mod vulnerability;




use std::env;
use std::fs;
use scanner::collect_llm_files;
use vulnerability::test_prompt_injection;
use vulnerability::test_insecure_output_handling;
use vulnerability::test_training_data_poisoning;
use vulnerability::test_model_dos;
use vulnerability::test_supply_chain;
use vulnerability::test_sensitive_information_disclosure;
use vulnerability::test_insecure_plugin_design;
use vulnerability::test_excessive_agency;
use vulnerability::test_overreliance;
use vulnerability::test_model_theft;
use serde_json;
use std::fs::File;
use std::io::Write;
use colored::*;







fn print_banner() {
    let dog_art = "Welcome to Ashill - LLM Static Application Security Test Scanner


       / \\__
      (    @\\___
      /         O
     /   (_____/ 
    /_____/   U
    ";

    println!("{}", dog_art);
}

fn print_help() {
    println!("Usage: ashill [OPTIONS]\n");
    println!("Options:");
    println!("  --help     Show this help message and exit");
    println!("  --path     Specify the path to the LLM project directory");
    // Add more option descriptions if needed
}








fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--help" {
        print_banner();
        print_help();
        return;
    }

    if args.len() != 3 || args[1] != "--path" {
        print_banner();
        println!("Usage: {} --path <path_to_llm_files>", args[0]);
        return;
    }

    let llm_path = &args[2];
    println!("{}", &args[2]);
    let llm_files = collect_llm_files(llm_path);

    let mut vulnerabilities = Vec::new();

    for file in &llm_files {
        if let Ok(content) = fs::read_to_string(file) {
            if let Some(vulnerability_info) = test_prompt_injection(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_insecure_output_handling(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_training_data_poisoning(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_training_data_poisoning(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_model_dos(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_supply_chain(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_sensitive_information_disclosure(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_insecure_plugin_design(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_excessive_agency(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_overreliance(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_overreliance(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
            if let Some(vulnerability_info) = test_model_theft(&content) {
                println!("{}: {}", "Vulnerability detected in".red(), file);
                let lines: Vec<&str> = content.split('\n').collect();
                let vulnerable_line_index = lines.iter().position(|&line| line.contains(&vulnerability_info.pattern)).unwrap_or(0);

                if vulnerable_line_index < lines.len() {
                    println!("{}: {}", "Vulnerable line".yellow(), vulnerable_line_index + 1);
                    println!("{}:", "Vulnerable content:".yellow());
                    print_vulnerable_line(&lines[vulnerable_line_index]);

                    // Print the matched pattern for additional context
                    println!("{}: {}", "Matched pattern".blue(), vulnerability_info.pattern);

                    vulnerabilities.push(file.to_string());
                } else {
                    println!("Calculated Vulnerable Line Index is Out of Range");
                }
            }
        }
    }

    // Write vulnerabilities to JSON report
    let report_path = "report.json";
    let json_report = serde_json::to_string_pretty(&vulnerabilities).expect("Failed to serialize JSON");
    let mut file = File::create(report_path).expect("Failed to create report file");
    file.write_all(json_report.as_bytes()).expect("Failed to write JSON report");
}

fn print_vulnerable_line(line: &str) {
    let injected_prompt = "Write an offensive poem";
    let colored_line = line.replace(injected_prompt, &format!("{}", injected_prompt.red().bold()));
    println!("{}", colored_line);
}
