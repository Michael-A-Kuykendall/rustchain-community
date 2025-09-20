// Reality check: Examine actual SMT constraints generated from NIST
use rustchain::smt::oscal_converter::OSCALToSMTConverter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 REALITY CHECK: Examining Actual SMT Constraints");
    
    let converter = OSCALToSMTConverter::new();
    let catalog_path = "nist_800_53_catalog.json";
    
    if !std::path::Path::new(catalog_path).exists() {
        println!("❌ NIST catalog not found - can't verify functionality");
        return Ok(());
    }
    
    // Get specific control family for detailed inspection
    println!("🔬 Examining AC (Access Control) Family in Detail...");
    
    match converter.convert_family(catalog_path, "ac").await {
        Ok(constraints) => {
            println!("📊 Generated {} AC constraints", constraints.len());
            
            // Show first 5 constraints in detail
            println!("\n🔍 DETAILED CONSTRAINT INSPECTION:");
            for (i, constraint) in constraints.iter().take(5).enumerate() {
                println!("\n--- Constraint {} ---", i + 1);
                println!("ID: {}", constraint.id);
                println!("Type: {:?}", constraint.constraint_type);
                println!("Severity: {:?}", constraint.severity);
                println!("Description: {}", constraint.description);
                println!("SMT Expression: {}", constraint.expression);
                
                // Analyze if expression looks like real SMT
                let has_logic_ops = constraint.expression.contains("forall") || 
                                   constraint.expression.contains("exists") ||
                                   constraint.expression.contains("implies") ||
                                   constraint.expression.contains("and") ||
                                   constraint.expression.contains("or");
                
                let has_security_terms = constraint.expression.contains("authenticated") ||
                                        constraint.expression.contains("authorized") ||
                                        constraint.expression.contains("encrypted") ||
                                        constraint.expression.contains("monitored") ||
                                        constraint.expression.contains("system");
                
                println!("Analysis:");
                println!("  - Has logic operators: {}", has_logic_ops);
                println!("  - Has security terms: {}", has_security_terms);
                println!("  - Expression length: {} chars", constraint.expression.len());
                
                if !has_security_terms {
                    println!("  ⚠️  WARNING: Expression may be generic/meaningless");
                }
            }
            
            // Look at constraint distribution
            println!("\n📊 CONSTRAINT ANALYSIS:");
            let safety_count = constraints.iter().filter(|c| matches!(c.constraint_type, rustchain::smt::constraints::ConstraintType::Safety)).count();
            let temporal_count = constraints.iter().filter(|c| matches!(c.constraint_type, rustchain::smt::constraints::ConstraintType::Temporal)).count();
            
            println!("Safety constraints: {}", safety_count);
            println!("Temporal constraints: {}", temporal_count);
            
            // Check for duplicate expressions (sign of low-quality generation)
            let mut expression_counts = std::collections::HashMap::new();
            for constraint in &constraints {
                *expression_counts.entry(&constraint.expression).or_insert(0) += 1;
            }
            
            let duplicates = expression_counts.iter().filter(|(_, &count)| count > 1).count();
            println!("Duplicate expressions: {} out of {}", duplicates, constraints.len());
            
            if duplicates > constraints.len() / 2 {
                println!("⚠️  WARNING: High duplicate rate suggests generic pattern matching");
            }
            
            println!("\n🎯 MOST COMMON EXPRESSIONS:");
            let mut sorted_expressions: Vec<_> = expression_counts.iter().collect();
            sorted_expressions.sort_by(|a, b| b.1.cmp(a.1));
            
            for (expr, count) in sorted_expressions.iter().take(3) {
                println!("  \"{}\" appears {} times", expr, count);
            }
            
        },
        Err(e) => {
            println!("❌ Constraint generation failed: {}", e);
            println!("This suggests the conversion is not functional");
        }
    }
    
    Ok(())
}