//! Test program for Bash Script parser
//! 
//! Simple CLI to test Bash script to RustChain mission conversion

use rustchain::transpiler::bash::BashParser;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <script.sh>", args[0]);
        std::process::exit(1);
    }
    
    let script_file = &args[1];
    
    match BashParser::parse_file(script_file).await {
        Ok(mission) => {
            println!("✅ Successfully parsed Bash script!");
            println!("Mission Name: {}", mission.name);
            if let Some(description) = &mission.description {
                println!("Description: {}", description);
            }
            println!("Steps: {}", mission.steps.len());
            
            for (i, step) in mission.steps.iter().enumerate() {
                println!("  {}: {} ({})", i + 1, step.name, format!("{:?}", step.step_type));
            }
            
            // Try to serialize to YAML
            match mission.to_yaml() {
                Ok(yaml_content) => {
                    println!("\n--- Generated RustChain Mission YAML ---");
                    println!("{}", yaml_content);
                },
                Err(e) => println!("❌ Failed to serialize to YAML: {}", e)
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to parse Bash script: {}", e);
            std::process::exit(1);
        }
    }
}