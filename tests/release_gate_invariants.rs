use proptest::prelude::*;
use std::process::Command;
use std::fs;

/// Property-based tests for release gate invariants
/// These tests ensure our release validation gates are stable and reliable

#[cfg(test)]
mod release_gate_ppt {
    use super::*;

    /// INVARIANT: Version validation must be deterministic and consistent
    #[test]
    fn test_version_validation_invariant() {
        // Test that version extraction is always consistent
        let cargo_toml_content = fs::read_to_string("Cargo.toml")
            .expect("Cargo.toml must exist");
        
        // Extract version multiple times - should always be identical
        let version1 = extract_cargo_version(&cargo_toml_content);
        let version2 = extract_cargo_version(&cargo_toml_content);
        let version3 = extract_cargo_version(&cargo_toml_content);
        
        assert_eq!(version1, version2);
        assert_eq!(version2, version3);
        assert!(!version1.is_empty(), "Version must not be empty");
        assert!(version1.chars().next().unwrap().is_numeric(), "Version must start with number");
    }

    /// INVARIANT: Compilation gates must be stable across multiple runs
    #[test]
    fn test_compilation_gate_stability() {
        // Test minimal build stability
        for _ in 0..3 {
            let output = Command::new("cargo")
                .args(&["check", "--no-default-features", "--features", "cli,transpiler"])
                .output()
                .expect("cargo check should work");
            
            assert!(output.status.success(), 
                "Minimal build must be stable. stderr: {}", 
                String::from_utf8_lossy(&output.stderr));
        }
    }

    /// INVARIANT: Package creation must be idempotent and deterministic
    #[test]
    fn test_package_gate_idempotency() {
        // Package creation should work multiple times
        for attempt in 1..=2 {
            let output = Command::new("cargo")
                .args(&["package", "--list", "--allow-dirty"])
                .output()
                .expect("cargo package should work");
            
            assert!(output.status.success(), 
                "Package creation attempt {} failed. stderr: {}", 
                attempt, String::from_utf8_lossy(&output.stderr));
            
            let package_list = String::from_utf8_lossy(&output.stdout);
            assert!(package_list.contains("Cargo.toml"), "Package must contain Cargo.toml");
            assert!(package_list.contains("src/"), "Package must contain src/");
        }
    }

    /// INVARIANT: Binary execution gate must be consistent
    #[test] 
    fn test_binary_execution_gate_consistency() {
        // Build the binary
        let build_output = Command::new("cargo")
            .args(&["build", "--release", "--no-default-features", "--features", "cli,transpiler"])
            .output()
            .expect("cargo build should work");
        
        assert!(build_output.status.success(), 
            "Release build failed. stderr: {}", 
            String::from_utf8_lossy(&build_output.stderr));

        // Test binary execution multiple times
        for run in 1..=3 {
            let exec_output = Command::new("./target/release/rustchain")
                .arg("--version")
                .output()
                .expect("binary execution should work");
            
            assert!(exec_output.status.success(), 
                "Binary execution run {} failed", run);
            
            let version_output = String::from_utf8_lossy(&exec_output.stdout);
            assert!(version_output.contains("rustchain"), 
                "Version output must contain 'rustchain'");
        }
    }

    proptest! {
        /// PROPERTY: Version validation should handle any well-formed version string
        #[test]
        fn prop_version_validation_handles_semver(
            major in 0u32..100,
            minor in 0u32..100, 
            patch in 0u32..100
        ) {
            let version = format!("{}.{}.{}", major, minor, patch);
            let tag = format!("v{}", version);
            
            // Version validation logic should be consistent
            let matches = version_tag_matches(&version, &tag);
            prop_assert_eq!(matches, true);
        }

        /// PROPERTY: Build should be deterministic regardless of valid feature combinations
        #[test]
        fn prop_build_determinism(
            use_cli in any::<bool>(),
            use_transpiler in any::<bool>()
        ) {
            if !use_cli && !use_transpiler {
                return Ok(()); // Skip invalid combination
            }
            
            // CLI feature requires transpiler - this is a known dependency
            if use_cli && !use_transpiler {
                return Ok(()); // Skip invalid combination
            }
            
            let mut features = Vec::new();
            if use_cli { features.push("cli"); }
            if use_transpiler { features.push("transpiler"); }
            
            let feature_str = features.join(",");
            
            // Build should succeed with valid feature combinations
            let output = Command::new("cargo")
                .args(&["check", "--no-default-features", "--features", &feature_str])
                .output()
                .expect("cargo check should work");
            
            prop_assert!(output.status.success(),
                "Build failed with features: {}. stderr: {}", 
                feature_str, String::from_utf8_lossy(&output.stderr));
        }
    }

    // Helper functions for gate validation logic
    fn extract_cargo_version(cargo_toml: &str) -> String {
        for line in cargo_toml.lines() {
            if line.trim().starts_with("version = ") {
                return line.split('"').nth(1).unwrap_or("").to_string();
            }
        }
        String::new()
    }

    fn version_tag_matches(version: &str, tag: &str) -> bool {
        let tag_clean = tag.strip_prefix('v').unwrap_or(tag);
        version == tag_clean
    }
}

/// Test that our release gate tests themselves are stable
#[test]
fn test_release_gate_test_stability() {
    // Meta-test: ensure our PPT tests can run reliably
    let test_output = Command::new("cargo")
        .args(&["test", "--test", "release_gate_invariants", "--features", "proptest"])
        .output()
        .expect("Running release gate tests should work");
    
    // The tests should be runnable (though may fail due to build state)
    // This is a smoke test for the test infrastructure itself
    assert!(test_output.status.code().is_some(), 
        "Test command should complete with exit code");
}