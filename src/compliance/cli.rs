//! Command Line Interface for Compliance SDK

use crate::compliance::ComplianceSystem;
use crate::core::Result;
use crate::engine::Mission;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Compliance CLI for enterprise compliance verification
#[derive(Parser)]
#[clap(name = "rustchain-compliance")]
#[clap(about = "Enterprise compliance verification for AI systems")]
pub struct ComplianceCLI {
    #[clap(subcommand)]
    pub command: ComplianceCommand,
}

#[derive(Subcommand)]
pub enum ComplianceCommand {
    /// List available compliance standards
    List,
    
    /// Verify mission against compliance standards
    Verify(VerifyArgs),
    
    /// Generate compliance report
    Report(ReportArgs),
    
    /// Initialize compliance system with NIST catalog
    Init(InitArgs),
    
    /// Show compliance statistics
    Stats,
    
    /// Convert OSCAL catalog to SMT constraints
    Convert(ConvertArgs),
}

#[derive(Args)]
pub struct VerifyArgs {
    /// Path to mission YAML file
    #[clap(short, long)]
    pub mission: PathBuf,
    
    /// Compliance standard to verify against
    #[clap(short, long, default_value = "GDPR")]
    pub standard: String,
    
    /// Output format (json, yaml, table)
    #[clap(short, long, default_value = "table")]
    pub format: String,
}

#[derive(Args)]
pub struct ReportArgs {
    /// Path to mission YAML file
    #[clap(short, long)]
    pub mission: PathBuf,
    
    /// Output file for report
    #[clap(short, long)]
    pub output: Option<PathBuf>,
    
    /// Include all standards in report
    #[clap(short, long)]
    pub all: bool,
    
    /// Report format (pdf, html, json, markdown)
    #[clap(short, long, default_value = "markdown")]
    pub format: String,
}

#[derive(Args)]
pub struct InitArgs {
    /// Path to NIST 800-53 OSCAL catalog JSON
    #[clap(short, long, default_value = "nist_800_53_catalog.json")]
    pub catalog: PathBuf,
    
    /// Download NIST catalog if not present
    #[clap(short, long)]
    pub download: bool,
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Path to OSCAL catalog JSON
    #[clap(short, long)]
    pub input: PathBuf,
    
    /// Output file for SMT constraints
    #[clap(short, long)]
    pub output: PathBuf,
    
    /// Convert only specific control family (AC, AU, etc.)
    #[clap(short, long)]
    pub family: Option<String>,
}

impl ComplianceCLI {
    pub async fn run(&self) -> Result<()> {
        let mut system = ComplianceSystem::new();
        
        match &self.command {
            ComplianceCommand::List => {
                self.list_standards(&system).await
            },
            ComplianceCommand::Verify(args) => {
                self.verify_mission(&system, args).await
            },
            ComplianceCommand::Report(args) => {
                self.generate_report(&system, args).await
            },
            ComplianceCommand::Init(args) => {
                self.initialize_system(&mut system, args).await
            },
            ComplianceCommand::Stats => {
                self.show_statistics(&mut system).await
            },
            ComplianceCommand::Convert(args) => {
                self.convert_catalog(&system, args).await
            },
        }
    }
    
    async fn list_standards(&self, system: &ComplianceSystem) -> Result<()> {
        println!("🔒 Available Compliance Standards:");
        
        for standard in system.list_standards() {
            let count = system.get_constraint_count(&standard);
            println!("   {} - {} constraints", standard, count);
        }
        
        Ok(())
    }
    
    async fn verify_mission(&self, system: &ComplianceSystem, args: &VerifyArgs) -> Result<()> {
        println!("🔍 Verifying mission: {:?}", args.mission);
        println!("📋 Standard: {}", args.standard);
        
        // Load mission
        let mission_content = std::fs::read_to_string(&args.mission)?;
        let mission: Mission = serde_yaml::from_str(&mission_content)
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to parse mission: {}", e)
                }
            ))?;
        
        // Verify compliance
        let report = system.verify_compliance(&args.standard, &mission).await?;
        
        // Output results
        match args.format.as_str() {
            "json" => println!("{}", serde_json::to_string_pretty(&report)?),
            "yaml" => println!("{}", serde_yaml::to_string(&report)?),
            "table" => report.print_table(),
            _ => {
                println!("❌ Unsupported format: {}", args.format);
                return Err(crate::core::error::RustChainError::Config(
                    crate::core::error::ConfigError::ParseError {
                        reason: "Unsupported output format".to_string()
                    }
                ));
            }
        }
        
        Ok(())
    }
    
    async fn generate_report(&self, system: &ComplianceSystem, args: &ReportArgs) -> Result<()> {
        println!("📊 Generating compliance report for: {:?}", args.mission);
        
        // Load mission
        let mission_content = std::fs::read_to_string(&args.mission)?;
        let mission: Mission = serde_yaml::from_str(&mission_content)
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to parse mission: {}", e)
                }
            ))?;
        
        // Generate reports
        let reports = if args.all {
            system.generate_comprehensive_report(&mission).await?
        } else {
            vec![system.verify_compliance("GDPR", &mission).await?]
        };
        
        // Format and output
        let formatted_report = self.format_reports(&reports, &args.format)?;
        
        if let Some(output_path) = &args.output {
            std::fs::write(output_path, formatted_report)?;
            println!("✅ Report saved to: {:?}", output_path);
        } else {
            println!("{}", formatted_report);
        }
        
        Ok(())
    }
    
    async fn initialize_system(&self, system: &mut ComplianceSystem, args: &InitArgs) -> Result<()> {
        println!("🚀 Initializing Compliance System...");
        
        // Download NIST catalog if requested
        if args.download && !args.catalog.exists() {
            println!("⬇️ Downloading NIST 800-53 catalog...");
            self.download_nist_catalog(&args.catalog).await?;
        }
        
        // Initialize system
        system.initialize().await?;
        
        println!("✅ Compliance system initialized successfully!");
        println!("📊 Available standards:");
        
        for standard in system.list_standards() {
            let count = system.get_constraint_count(&standard);
            println!("   {} - {} constraints", standard, count);
        }
        
        Ok(())
    }
    
    async fn show_statistics(&self, system: &mut ComplianceSystem) -> Result<()> {
        println!("📊 Compliance System Statistics:");
        
        let _ = system.initialize().await; // Ensure initialized
        
        let standards = system.list_standards();
        println!("   Total Standards: {}", standards.len());
        
        let total_constraints: usize = standards.iter()
            .map(|s| system.get_constraint_count(s))
            .sum();
        println!("   Total Constraints: {}", total_constraints);
        
        println!("\n📋 Standards Breakdown:");
        for standard in standards {
            let count = system.get_constraint_count(&standard);
            println!("   {}: {} constraints", standard, count);
        }
        
        // Show memory usage estimate
        let memory_mb = total_constraints * 200 / 1024 / 1024; // Rough estimate
        println!("\n💾 Estimated Memory Usage: {} MB", memory_mb);
        
        Ok(())
    }
    
    async fn convert_catalog(&self, system: &ComplianceSystem, args: &ConvertArgs) -> Result<()> {
        println!("🔄 Converting OSCAL catalog: {:?}", args.input);
        
        let constraints = if let Some(family) = &args.family {
            println!("🎯 Converting family: {}", family);
            system.oscal_converter.convert_family(
                args.input.to_str().ok_or_else(|| {
                    anyhow::anyhow!("Invalid Unicode in input path: {:?}", args.input)
                })?, 
                family
            ).await?
        } else {
            println!("🌐 Converting entire catalog...");
            system.oscal_converter.convert_nist_catalog(
                args.input.to_str().ok_or_else(|| {
                    anyhow::anyhow!("Invalid Unicode in input path: {:?}", args.input)
                })?
            ).await?
        };
        
        // Save constraints
        let constraints_json = serde_json::to_string_pretty(&constraints)?;
        std::fs::write(&args.output, constraints_json)?;
        
        println!("✅ Converted {} constraints to: {:?}", constraints.len(), args.output);
        
        Ok(())
    }
    
    async fn download_nist_catalog(&self, output_path: &PathBuf) -> Result<()> {
        use reqwest;
        
        let url = "https://raw.githubusercontent.com/usnistgov/OSCAL/main/src/content/nist.gov/SP800-53/rev5/json/NIST_SP-800-53_rev5_catalog.json";
        
        println!("📥 Downloading from: {}", url);
        
        let response = reqwest::get(url).await
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to download catalog: {}", e)
                }
            ))?;
        
        let content = response.text().await
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to read response: {}", e)
                }
            ))?;
        
        std::fs::write(output_path, content)?;
        println!("✅ NIST catalog downloaded to: {:?}", output_path);
        
        Ok(())
    }
    
    fn format_reports(&self, reports: &[crate::compliance::ComplianceReport], format: &str) -> Result<String> {
        match format {
            "json" => Ok(serde_json::to_string_pretty(reports)?),
            "yaml" => Ok(serde_yaml::to_string(reports)?),
            "markdown" => Ok(self.format_markdown_reports(reports)),
            "html" => Ok(self.format_html_reports(reports)),
            _ => Err(crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Unsupported report format: {}", format)
                }
            ))
        }
    }
    
    fn format_markdown_reports(&self, reports: &[crate::compliance::ComplianceReport]) -> String {
        let mut output = String::new();
        output.push_str("# Compliance Report\n\n");
        
        for report in reports {
            output.push_str(&format!("## {} Compliance\n\n", report.standard));
            output.push_str(&format!("- **Status**: {}\n", if report.compliant { "✅ COMPLIANT" } else { "❌ NON-COMPLIANT" }));
            output.push_str(&format!("- **Risk Score**: {:.2}\n", report.risk_score));
            output.push_str(&format!("- **Violations**: {}\n\n", report.violations.len()));
            
            if !report.violations.is_empty() {
                output.push_str("### Violations\n\n");
                for violation in &report.violations {
                    output.push_str(&format!("- **{}**: {}\n", violation.severity, violation.description));
                }
                output.push_str("\n");
            }
        }
        
        output
    }
    
    fn format_html_reports(&self, reports: &[crate::compliance::ComplianceReport]) -> String {
        let mut output = String::new();
        output.push_str("<!DOCTYPE html>\n<html><head><title>Compliance Report</title></head><body>\n");
        output.push_str("<h1>Compliance Report</h1>\n");
        
        for report in reports {
            let status_color = if report.compliant { "green" } else { "red" };
            let status_text = if report.compliant { "COMPLIANT" } else { "NON-COMPLIANT" };
            
            output.push_str(&format!("<h2>{} Compliance</h2>\n", report.standard));
            output.push_str(&format!("<p><strong>Status</strong>: <span style=\"color: {}\">{}</span></p>\n", status_color, status_text));
            output.push_str(&format!("<p><strong>Risk Score</strong>: {:.2}</p>\n", report.risk_score));
            output.push_str(&format!("<p><strong>Violations</strong>: {}</p>\n", report.violations.len()));
            
            if !report.violations.is_empty() {
                output.push_str("<h3>Violations</h3>\n<ul>\n");
                for violation in &report.violations {
                    output.push_str(&format!("<li><strong>{}</strong>: {}</li>\n", violation.severity, violation.description));
                }
                output.push_str("</ul>\n");
            }
        }
        
        output.push_str("</body></html>\n");
        output
    }
}

/// Enterprise CLI runner for compliance verification
pub async fn run_compliance_cli() -> Result<()> {
    let cli = ComplianceCLI::parse();
    cli.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parsing() {
        // Test that CLI can be constructed
        let args = vec!["rustchain-compliance", "list"];
        let cli = ComplianceCLI::try_parse_from(args);
        assert!(cli.is_ok());
    }
}