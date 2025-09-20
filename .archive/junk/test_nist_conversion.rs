// Test NIST 800-53 conversion capabilities
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use rustchain_community::smt::oscal_converter::OSCALToSMTConverter;
    
    println!("🔍 Testing NIST 800-53 OSCAL Conversion...");
    
    let converter = OSCALToSMTConverter::new();
    
    // Test if our downloaded catalog exists
    let catalog_path = "nist_800_53_catalog.json";
    if !std::path::Path::new(catalog_path).exists() {
        println!("❌ NIST catalog not found at {}", catalog_path);
        return Ok(());
    }
    
    println!("✅ NIST catalog found, analyzing...");
    
    // Count total controls
    match converter.count_controls(catalog_path).await {
        Ok((total, families)) => {
            println!("📊 NIST 800-53 Analysis Results:");
            println!("   Total Controls: {}", total);
            println!("   Control Families: {}", families.len());
            
            for (family, count) in families.iter() {
                println!("   {} family: {} controls", family.to_uppercase(), count);
            }
            
            // Test conversion of AC family
            println!("\n🔬 Testing AC (Access Control) Family Conversion...");
            match converter.convert_family(catalog_path, "ac").await {
                Ok(constraints) => {
                    println!("✅ Generated {} SMT constraints for AC family", constraints.len());
                    
                    // Show first few constraints
                    for (i, constraint) in constraints.iter().take(3).enumerate() {
                        println!("   {}. {} - {}", i+1, constraint.id, constraint.description);
                    }
                },
                Err(e) => println!("❌ AC family conversion failed: {}", e),
            }
            
            // Test full catalog conversion
            println!("\n🚀 Testing Full Catalog Conversion...");
            match converter.convert_nist_catalog(catalog_path).await {
                Ok(all_constraints) => {
                    println!("🎉 SUCCESS: Generated {} total SMT constraints from NIST 800-53!", all_constraints.len());
                    
                    // Analyze by constraint type
                    let safety_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, rustchain_community::smt::constraints::ConstraintType::Safety)).count();
                    let temporal_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, rustchain_community::smt::constraints::ConstraintType::Temporal)).count();
                    
                    println!("   Safety constraints: {}", safety_count);
                    println!("   Temporal constraints: {}", temporal_count);
                    
                    // Analyze by severity
                    let critical_count = all_constraints.iter().filter(|c| matches!(c.severity, rustchain_community::smt::constraints::ConstraintSeverity::Critical)).count();
                    let high_count = all_constraints.iter().filter(|c| matches!(c.severity, rustchain_community::smt::constraints::ConstraintSeverity::High)).count();
                    let medium_count = all_constraints.iter().filter(|c| matches!(c.severity, rustchain_community::smt::constraints::ConstraintSeverity::Medium)).count();
                    
                    println!("   Critical: {}, High: {}, Medium: {}", critical_count, high_count, medium_count);
                },
                Err(e) => println!("❌ Full conversion failed: {}", e),
            }
        },
        Err(e) => println!("❌ Failed to count controls: {}", e),
    }
    
    Ok(())
}