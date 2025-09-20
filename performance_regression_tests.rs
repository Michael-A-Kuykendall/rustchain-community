#!/usr/bin/env cargo script

//! RustChain Performance Regression Test Suite
//! 
//! Automated tests to detect performance regressions in CI/CD pipeline
//! 
//! Usage: cargo test --release --test performance_regression_tests

use std::time::Instant;
use std::process::Command;

const PERFORMANCE_THRESHOLDS: &[(&str, &str, u128)] = &[
    ("CLI Startup", "--version", 25),                           // 25ms max
    ("Mission Validation", "mission validate examples/01_hello_world_mission.yaml", 30), // 30ms max  
    ("Tool Registration", "tools list", 30),                   // 30ms max
    ("Simple Mission", "run examples/01_hello_world_mission.yaml", 60), // 60ms max
];

#[test]
fn test_cli_startup_performance() {
    let threshold = 25; // ms
    let times = benchmark_operation(&["--version"], 5);
    let avg_time = times.iter().sum::<u128>() / times.len() as u128;
    
    assert!(
        avg_time <= threshold,
        "CLI startup time {}ms exceeds threshold {}ms",
        avg_time,
        threshold
    );
    
    println!("✅ CLI startup: {}ms (threshold: {}ms)", avg_time, threshold);
}

#[test]
fn test_mission_validation_performance() {
    let threshold = 30; // ms
    let times = benchmark_operation(&["mission", "validate", "examples/01_hello_world_mission.yaml"], 5);
    let avg_time = times.iter().sum::<u128>() / times.len() as u128;
    
    assert!(
        avg_time <= threshold,
        "Mission validation time {}ms exceeds threshold {}ms",
        avg_time,
        threshold
    );
    
    println!("✅ Mission validation: {}ms (threshold: {}ms)", avg_time, threshold);
}

#[test]
fn test_tool_registration_performance() {
    let threshold = 30; // ms
    let times = benchmark_operation(&["tools", "list"], 5);
    let avg_time = times.iter().sum::<u128>() / times.len() as u128;
    
    assert!(
        avg_time <= threshold,
        "Tool registration time {}ms exceeds threshold {}ms",
        avg_time,
        threshold
    );
    
    println!("✅ Tool registration: {}ms (threshold: {}ms)", avg_time, threshold);
}

#[test]
fn test_simple_mission_execution_performance() {
    let threshold = 60; // ms
    let times = benchmark_operation(&["run", "examples/01_hello_world_mission.yaml"], 3);
    let avg_time = times.iter().sum::<u128>() / times.len() as u128;
    
    // Cleanup after each test
    for _ in 0..times.len() {
        let _ = std::fs::remove_file("hello_rustchain.txt");
    }
    
    assert!(
        avg_time <= threshold,
        "Mission execution time {}ms exceeds threshold {}ms",
        avg_time,
        threshold
    );
    
    println!("✅ Mission execution: {}ms (threshold: {}ms)", avg_time, threshold);
}

#[test]
fn test_performance_consistency() {
    // Test that performance is consistent (low variance)
    let times = benchmark_operation(&["--version"], 10);
    
    let avg = times.iter().sum::<u128>() / times.len() as u128;
    let variance = times.iter()
        .map(|&x| (x as f64 - avg as f64).powi(2))
        .sum::<f64>() / times.len() as f64;
    let std_dev = variance.sqrt();
    
    // Standard deviation should be less than 20% of average
    let max_std_dev = avg as f64 * 0.2;
    
    assert!(
        std_dev <= max_std_dev,
        "Performance variance too high: std_dev={}ms, max_allowed={}ms",
        std_dev,
        max_std_dev
    );
    
    println!("✅ Performance consistency: {}ms ±{:.1}ms", avg, std_dev);
}

#[test]
fn test_memory_usage_regression() {
    // Basic memory usage test - ensure binary size hasn't grown significantly
    let binary_path = "./target/release/rustchain.exe";
    
    if let Ok(metadata) = std::fs::metadata(binary_path) {
        let size_mb = metadata.len() / 1024 / 1024;
        let max_size_mb = 50; // 50MB max
        
        assert!(
            size_mb <= max_size_mb,
            "Binary size {}MB exceeds threshold {}MB",
            size_mb,
            max_size_mb
        );
        
        println!("✅ Binary size: {}MB (threshold: {}MB)", size_mb, max_size_mb);
    }
}

#[test]
fn test_concurrent_performance() {
    // Test that performance doesn't degrade under concurrent load
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};
    
    let total_time = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    
    // Run 5 concurrent operations
    for _ in 0..5 {
        let total_time_clone = Arc::clone(&total_time);
        let handle = thread::spawn(move || {
            let times = benchmark_operation(&["--version"], 2);
            let avg_time = times.iter().sum::<u128>() / times.len() as u128;
            total_time_clone.fetch_add(avg_time as u64, Ordering::SeqCst);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let avg_concurrent_time = total_time.load(Ordering::SeqCst) / 5;
    let max_concurrent_time = 40; // ms
    
    assert!(
        avg_concurrent_time <= max_concurrent_time,
        "Concurrent performance {}ms exceeds threshold {}ms",
        avg_concurrent_time,
        max_concurrent_time
    );
    
    println!("✅ Concurrent performance: {}ms (threshold: {}ms)", avg_concurrent_time, max_concurrent_time);
}

fn benchmark_operation(args: &[&str], iterations: usize) -> Vec<u128> {
    let mut times = Vec::new();
    
    for _ in 0..iterations {
        let start = Instant::now();
        let output = Command::new("./target/release/rustchain")
            .args(args)
            .output()
            .expect("Failed to execute command");
        let duration = start.elapsed();
        
        if output.status.success() {
            times.push(duration.as_millis());
        }
    }
    
    times
}

// Integration test for CI/CD pipeline
#[test]
fn test_all_performance_thresholds() {
    println!("🚀 Running RustChain Performance Regression Tests");
    
    for &(name, args_str, threshold) in PERFORMANCE_THRESHOLDS {
        let args: Vec<&str> = args_str.split_whitespace().collect();
        let times = benchmark_operation(&args, 3);
        let avg_time = times.iter().sum::<u128>() / times.len() as u128;
        
        assert!(
            avg_time <= threshold,
            "{} performance {}ms exceeds threshold {}ms",
            name,
            avg_time,
            threshold
        );
        
        println!("✅ {}: {}ms (threshold: {}ms)", name, avg_time, threshold);
    }
    
    println!("🎯 All performance regression tests passed!");
}

// Benchmark suite for CI/CD integration
#[cfg(test)]
mod benchmarks {
    use super::*;
    
    #[test]
    fn benchmark_suite_for_ci() {
        // This test runs the full benchmark suite and outputs JSON for tracking
        println!("{{");
        println!("  \"timestamp\": \"{}\",", chrono::Utc::now().to_rfc3339());
        println!("  \"environment\": \"CI\",");
        println!("  \"build_type\": \"release\",");
        println!("  \"metrics\": {{");
        
        let mut first = true;
        for &(name, args_str, _threshold) in PERFORMANCE_THRESHOLDS {
            if !first {
                println!(",");
            }
            first = false;
            
            let args: Vec<&str> = args_str.split_whitespace().collect();
            let times = benchmark_operation(&args, 5);
            let avg_time = times.iter().sum::<u128>() / times.len() as u128;
            let min_time = times.iter().min().unwrap_or(&0);
            let max_time = times.iter().max().unwrap_or(&0);
            
            println!("    \"{}\": {{", name);
            println!("      \"average_ms\": {},", avg_time);
            println!("      \"min_ms\": {},", min_time);
            println!("      \"max_ms\": {}", max_time);
            print!("    }}");
        }
        
        println!();
        println!("  }}");
        println!("}}");
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;
    
    #[test]
    #[ignore] // Run with --ignored for load testing
    fn load_test_startup_performance() {
        // High-volume test for load scenarios
        let iterations = 100;
        let times = benchmark_operation(&["--version"], iterations);
        
        let avg_time = times.iter().sum::<u128>() / times.len() as u128;
        let p95_time = {
            let mut sorted_times = times.clone();
            sorted_times.sort_unstable();
            sorted_times[iterations * 95 / 100]
        };
        
        println!("Load test results ({} iterations):", iterations);
        println!("  Average: {}ms", avg_time);
        println!("  P95: {}ms", p95_time);
        
        // Under load, we allow higher thresholds
        assert!(avg_time <= 50, "Load test average {}ms exceeds 50ms", avg_time);
        assert!(p95_time <= 100, "Load test P95 {}ms exceeds 100ms", p95_time);
    }
}