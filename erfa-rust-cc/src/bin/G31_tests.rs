use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

#[derive(Clone)]
struct TestResult {
    name: String,
    status: String,
    message: Option<String>,
    duration: std::time::Duration,
}

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn show_help(program: &str) {
    println!("Usage: {} [-v]", program);
    println!("  -v    Verbose mode - show detailed output from each test");
    println!("  -h    Show this help message");
}

fn get_exe_dir() -> PathBuf {
    let mut exe_path = env::current_exe().expect("Fatal: Could not determine executable path");
    exe_path.pop();
    exe_path
}

fn run_test_verbose(test_name: &str, binary_path: &PathBuf) -> TestResult {
    println!("\n{}--- Running {} ---{}", CYAN, test_name, RESET);

    let test_start = Instant::now();
    let status = Command::new(binary_path).arg("verbose").status();

    let test_duration = test_start.elapsed();

    match status {
        Ok(status) if status.success() => TestResult {
            name: test_name.to_string(),
            status: "PASSED".to_string(),
            message: None,
            duration: test_duration,
        },
        Ok(_) => TestResult {
            name: test_name.to_string(),
            status: "FAILED".to_string(),
            message: Some("Test failed - see output above".to_string()),
            duration: test_duration,
        },
        Err(e) => {
            eprintln!(
                "{}Error executing '{}': {}{}",
                RED,
                binary_path.display(),
                e,
                RESET
            );
            TestResult {
                name: test_name.to_string(),
                status: "ERROR".to_string(),
                message: Some(format!("Execution error: {}", e)),
                duration: test_duration,
            }
        }
    }
}

fn run_test_quiet(test_name: &str, binary_path: &PathBuf) -> TestResult {
    print!("Running {:.<25}", format!("{} ", test_name));
    io::stdout().flush().unwrap();

    let test_start = Instant::now();
    let output = Command::new(binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let test_duration = test_start.elapsed();

    let result = match output {
        Ok(output) if output.status.success() => {
            println!("{}[ PASSED ]{}", GREEN, RESET);
            TestResult {
                name: test_name.to_string(),
                status: "PASSED".to_string(),
                message: None,
                duration: test_duration,
            }
        }
        Ok(output) => {
            println!("{}[ FAILED ]{}", RED, RESET);

            let stderr_str = String::from_utf8_lossy(&output.stderr);
            let stdout_str = String::from_utf8_lossy(&output.stdout);

            let error_msg = stderr_str
                .lines()
                .next()
                .or_else(|| stdout_str.lines().find(|line| line.contains("failed")))
                .unwrap_or("Test failed with no output")
                .to_string();

            println!(
                "{}--- Failure details for {} ---{}",
                YELLOW, test_name, RESET
            );
            Command::new(binary_path)
                .arg("verbose")
                .status()
                .expect("Failed to re-run for failure details");
            println!("{}------------------------------------{}", YELLOW, RESET);

            TestResult {
                name: test_name.to_string(),
                status: "FAILED".to_string(),
                message: Some(error_msg),
                duration: test_duration,
            }
        }
        Err(e) => {
            println!("{}[ ERROR ]{}", RED, RESET);
            eprintln!(
                "{}Error executing '{}': {}{}",
                RED,
                binary_path.display(),
                e,
                RESET
            );
            TestResult {
                name: test_name.to_string(),
                status: "ERROR".to_string(),
                message: Some(format!("Execution error: {}", e)),
                duration: test_duration,
            }
        }
    };

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "-h" | "--help" | "-help"))
    {
        show_help(&args[0]);
        std::process::exit(0);
    }

    let verbose = args.iter().any(|arg| arg == "-v");
    let exe_dir = get_exe_dir();

    let test_binaries = [
        "G1_tests",
        "G2_tests",
        "G3_tests",
        "G4_tests",
        "G5_tests",
        "G6_tests",
        "G7_tests",
        "G8_tests",
        "G9_tests",
        "G10_tests",
        "G11_tests",
        "G12_tests",
        "G13_tests",
        "G14_tests",
        "G15_tests",
        "G16_tests",
        "G17_tests",
        "G18_tests",
        "G19_tests",
        "G20_tests",
        "G21_tests",
        "G22_tests",
        "G23_tests",
        "G24_tests",
        "G25_tests",
        "G26_tests",
        "G27_tests",
        "G28_tests",
        "G29_tests",
        "G30_tests",
        "G32_tests",
        "G33_tests",
        "G34_tests",
        "G35_tests",
        "t_erfa_c_extra",
    ];

    println!(
        "{}=======================================================",
        BOLD
    );
    println!(
        "  Running Full ERFA Test Suite ({} total tests)",
        test_binaries.len()
    );
    println!("  Running from: {}", exe_dir.display());
    if verbose {
        println!("  Mode: {}VERBOSE (-v){}", CYAN, RESET);
    }
    println!(
        "======================================================={}",
        RESET
    );

    let start_time = Instant::now();

    let results: Vec<TestResult> = test_binaries
        .iter()
        .map(|&test_name| {
            let mut binary_path = PathBuf::from(&exe_dir);
            binary_path.push(test_name);

            if verbose {
                run_test_verbose(test_name, &binary_path)
            } else {
                run_test_quiet(test_name, &binary_path)
            }
        })
        .collect();

    let total_passed = results.iter().filter(|r| r.status == "PASSED").count();
    let total_failed = results.len() - total_passed;

    println!("\n{}=== Detailed Test Results ==={}", BOLD, RESET);
    println!(
        "{:<20} {:<10} {:<10} {}",
        "Test", "Status", "Time(ms)", "Message"
    );
    println!("{}", "-".repeat(80));

    for result in &results {
        let (status_color, status_display) = match result.status.as_str() {
            "PASSED" => (GREEN, result.status.clone()),
            "FAILED" => (RED, format!("*{}*", result.status)),
            "ERROR" => (YELLOW, format!("!{}!", result.status)),
            _ => ("", result.status.clone()),
        };

        println!(
            "{:<20} {}{:<10}{} {:<10.1} {}",
            result.name,
            status_color,
            status_display,
            RESET,
            result.duration.as_millis() as f64,
            result.message.as_ref().unwrap_or(&String::new())
        );
    }

    let duration = start_time.elapsed();
    let (summary_color, summary_status) = if total_failed == 0 {
        (GREEN, "SUCCESS")
    } else {
        (RED, "FAILURE")
    };

    println!(
        "\n{}=======================================================",
        BOLD
    );
    println!(
        "  Test Suite Result: {}{}{}",
        summary_color, summary_status, RESET
    );
    println!("-------------------------------------------------------");
    println!(
        "  Summary: {}{}{} passed, {}{}{} failed out of {} tests.",
        GREEN,
        total_passed,
        RESET,
        RED,
        total_failed,
        RESET,
        test_binaries.len()
    );
    println!("  Total time: {:.2}s", duration.as_secs_f64());
    println!(
        "======================================================={}",
        RESET
    );

    std::process::exit(if total_failed > 0 { 1 } else { 0 });
}
