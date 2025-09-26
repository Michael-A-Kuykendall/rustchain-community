use clap::Parser;
use rustchain::cli::commands_pretty::Cli;
use rustchain::cli::handlers_pretty::PrettyCliHandler;
use rustchain::core::config::Config;
use std::process;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Initialize logging with clean output
    init_clean_logging();
    
    // Load configuration
    let config = Config::default(); // You might want to load from file
    
    // Create beautiful CLI handler
    let handler = PrettyCliHandler::new(config);
    
    // Handle the command with beautiful output
    if let Err(e) = handler.handle(cli).await {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

/// Initialize clean logging that doesn't interfere with pretty output
fn init_clean_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    // Set default log level if not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn"); // Only show warnings and errors
    }
    
    // Initialize with a clean format
    let _result = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_file(false)
                .with_line_number(false)
                .compact()
        )
        .try_init();
}