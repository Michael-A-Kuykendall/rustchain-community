// Quick test to count NIST controls
use std::process::Command;

fn main() {
    println!("Testing OSCAL converter...");
    
    // Use our OSCAL converter to count controls
    let output = Command::new("cargo")
        .args(&["run", "--features", "smt", "--bin", "rustchain", "--", "smt", "count-controls", "nist_800_53_catalog.json"])
        .output();
        
    match output {
        Ok(result) => {
            println!("STDOUT: {}", String::from_utf8_lossy(&result.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&result.stderr));
        }
        Err(e) => println!("Error: {}", e),
    }
}