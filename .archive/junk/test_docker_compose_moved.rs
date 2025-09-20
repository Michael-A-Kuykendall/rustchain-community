//! Test program for Docker Compose parser
//! 
//! Simple CLI to test Docker Compose to RustChain mission conversion

use rustchain::transpiler::docker_compose::DockerComposeParser;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <docker-compose.yaml>", args[0]);
        std::process::exit(1);
    }
    
    let compose_file = &args[1];
    
    match DockerComposeParser::parse_file(compose_file).await {
        Ok(mission) => {
            println!("✅ Successfully parsed Docker Compose file!");
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
            eprintln!("❌ Failed to parse Docker Compose file: {}", e);
            std::process::exit(1);
        }
    }
}