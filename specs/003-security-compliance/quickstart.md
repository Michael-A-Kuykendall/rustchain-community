# Security & Compliance System - Quick Start Guide

**Feature**: Security & Compliance System  
**Specification**: [spec.md](./spec.md)  
**Technical Plan**: [plan.md](./plan.md)  
**Implementation Tasks**: [tasks.md](./tasks.md)  

## 🚀 5-Minute Quick Start

### Prerequisites
- Rust 1.70+ with Cargo
- PostgreSQL 14+ (for audit storage)
- Redis 6+ (for session management)
- OpenSSL/LibreSSL (for cryptographic operations)

### 1. Initialize Security Configuration

```rust
use rustchain_security::{SecurityManager, SecurityConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment
    let config = SecurityConfig {
        jwt_secret: Some("your-jwt-secret-key".to_string()),
        session_timeout_minutes: 60,
        max_login_attempts: 3,
        encryption_algorithm: "AES-256-GCM".to_string(),
        compliance_frameworks: vec!["GDPR".to_string(), "SOC2".to_string()],
        threat_detection_enabled: true,
        audit_retention_days: 2555, // 7 years
    };
    
    // Initialize security manager
    let security_manager = SecurityManager::new(config)?;
    
    println!("✅ Security system initialized successfully!");
    Ok(())
}
```

### 2. Basic Authentication Flow

```rust
use rustchain_security::{Credentials, SecurityContext};

// Authenticate user with username/password
let credentials = Credentials::UsernamePassword {
    username: "admin@example.com".to_string(),
    password: "secure_password_123".to_string(),
    mfa_code: None,
};

// Perform authentication
let security_context = security_manager.authenticate(&credentials).await?;
println!("🔐 User authenticated: {}", security_context.user_id.unwrap());

// Check authorization for resource access
let authorized = security_manager
    .authorize(&security_context, "missions", "execute")
    .await?;

if authorized {
    println!("✅ User authorized to execute missions");
} else {
    println!("❌ Access denied");
}
```

### 3. Enable Multi-Factor Authentication

```rust
use rustchain_security::mfa::{MfaProvider, TotpService};

// Setup TOTP-based MFA
let mfa_provider = MfaProvider::new()?;
let totp_secret = mfa_provider.setup_totp("user@example.com").await?;

println!("📱 TOTP Secret: {}", totp_secret);
println!("📱 Add this to your authenticator app");

// Authenticate with MFA
let credentials_with_mfa = Credentials::UsernamePassword {
    username: "user@example.com".to_string(),
    password: "password123".to_string(),
    mfa_code: Some("123456".to_string()), // From authenticator app
};

let context = security_manager.authenticate(&credentials_with_mfa).await?;
println!("🔐 MFA authentication successful!");
```

### 4. Audit Trail Verification

```rust
use rustchain_security::audit::{SecurityEvent, AuditService};

// Log a security event
let event = SecurityEvent::DataAccess {
    user_id: "admin@example.com".to_string(),
    resource: "customer_data".to_string(),
    operation: "read".to_string(),
    classification: SecurityLevel::Confidential,
};

// Event is automatically logged and cryptographically signed
security_manager.log_security_event(event).await?;

// Verify audit trail integrity
let integrity_check = security_manager
    .verify_audit_integrity(0, 100)
    .await?;

if integrity_check {
    println!("✅ Audit trail integrity verified");
} else {
    println!("⚠️ Audit trail tampering detected!");
}
```

### 5. Compliance Reporting

```rust
use rustchain_security::compliance::{ComplianceFramework, ComplianceReport};

// Generate GDPR compliance report
let gdpr_report = security_manager
    .generate_compliance_report("GDPR")
    .await?;

println!("📊 GDPR Compliance Score: {:.1}%", gdpr_report.overall_score * 100.0);
println!("📋 Total Controls: {}", gdpr_report.control_assessments.len());

// Generate SOC2 compliance report
let soc2_report = security_manager
    .generate_compliance_report("SOC2") 
    .await?;

println!("📊 SOC2 Compliance Score: {:.1}%", soc2_report.overall_score * 100.0);
```

### 6. Real-Time Threat Detection

```rust
use rustchain_security::threats::{ThreatDetectionService, UserActivity};

// Enable behavioral analytics
let activities = vec![
    UserActivity {
        user_id: "admin@example.com".to_string(),
        action: "login".to_string(),
        resource: "dashboard".to_string(),
        timestamp: chrono::Utc::now(),
        ip_address: Some("192.168.1.100".to_string()),
        user_agent: Some("Mozilla/5.0...".to_string()),
    }
];

// Analyze for threats
let threat_analysis = security_manager
    .analyze_threats(&activities)
    .await?;

if !threat_analysis.threats_detected.is_empty() {
    println!("🚨 Threats detected: {}", threat_analysis.threats_detected.len());
    for threat in &threat_analysis.threats_detected {
        println!("   - {}: {} ({})", threat.threat_type, threat.description, threat.severity);
    }
} else {
    println!("✅ No threats detected");
}
```

### 7. Privacy Controls (GDPR Article 17)

```rust
use rustchain_security::privacy::{DataErasureService, ErasureMethod};

// Process "right to be forgotten" request
let erasure_request = DataErasureRequest {
    data_subject_id: "user@example.com".to_string(),
    erasure_method: ErasureMethod::CryptographicErasure,
    legal_basis: "GDPR Article 17 - Right to Erasure".to_string(),
    verification_required: true,
};

let erasure_result = security_manager
    .process_data_erasure(erasure_request)
    .await?;

println!("🗑️ Data erasure completed");
println!("🔐 Verification hash: {}", erasure_result.verification_hash);
println!("📄 Proof of erasure: {}", erasure_result.proof_of_erasure);
```

### 8. Security Metrics Dashboard

```rust
use rustchain_security::metrics::SecurityMetrics;

// Get real-time security metrics
let metrics = security_manager.get_security_metrics().await?;

println!("📊 Security Metrics (24h):");
println!("   Authentication events: {}", metrics.authentication_events_24h);
println!("   Threats detected: {}", metrics.threats_detected_24h);
println!("   Policy violations: {}", metrics.policy_violations_24h);
println!("   Active sessions: {}", metrics.active_sessions);
```

## 🛠️ Configuration Options

### Environment Variables

```bash
# Database Configuration
export DATABASE_URL="postgresql://user:password@localhost/rustchain_security"
export REDIS_URL="redis://localhost:6379"

# Security Configuration  
export JWT_SECRET="your-256-bit-secret-key"
export SESSION_TIMEOUT_MINUTES="60"
export MAX_LOGIN_ATTEMPTS="3"

# Compliance Configuration
export COMPLIANCE_FRAMEWORKS="GDPR,SOC2,ISO27001"
export AUDIT_RETENTION_DAYS="2555"

# Threat Detection
export THREAT_DETECTION_ENABLED="true"
export BEHAVIORAL_ANALYTICS_ENABLED="true"

# SIEM Integration
export SIEM_ENABLED="true"
export SIEM_TYPE="splunk"
export SIEM_ENDPOINT="https://splunk.company.com:8088"
export SIEM_TOKEN="your-hec-token"
```

### Configuration File (rustchain-security.toml)

```toml
[security]
jwt_secret = "your-jwt-secret-key"
session_timeout_minutes = 60
max_login_attempts = 3
encryption_algorithm = "AES-256-GCM"

[compliance]
frameworks = ["GDPR", "SOC2", "ISO27001", "NIST"]
audit_retention_days = 2555
evidence_collection_enabled = true

[threat_detection]
enabled = true
behavioral_analytics = true
ml_detection = true
threat_intelligence_feeds = [
    "https://api.threatfeed.com/indicators"
]

[privacy]
data_retention_enabled = true
automatic_erasure = true
consent_management = true

[integrations]
siem_enabled = true
siem_type = "splunk"
identity_providers = ["azure_ad", "okta"]

[performance]
audit_batch_size = 1000
session_cleanup_interval = 300
cache_ttl = 3600
```

## 🔧 Common Configuration Patterns

### High-Security Environment

```rust
let high_security_config = SecurityConfig {
    jwt_secret: Some(env::var("JWT_SECRET")?),
    session_timeout_minutes: 15, // Shorter sessions
    max_login_attempts: 2, // Stricter lockout
    encryption_algorithm: "ChaCha20-Poly1305".to_string(), // Alternative cipher
    compliance_frameworks: vec![
        "GDPR".to_string(),
        "SOX".to_string(), 
        "ISO27001".to_string(),
        "NIST".to_string()
    ],
    threat_detection_enabled: true,
    audit_retention_days: 3650, // 10 years
};
```

### Development Environment

```rust
let dev_config = SecurityConfig {
    jwt_secret: Some("dev-secret-key-not-for-production".to_string()),
    session_timeout_minutes: 480, // 8 hours for development
    max_login_attempts: 10, // More lenient for testing
    encryption_algorithm: "AES-256-GCM".to_string(),
    compliance_frameworks: vec!["GDPR".to_string()], // Minimal for dev
    threat_detection_enabled: false, // Disable in dev
    audit_retention_days: 30, // Short retention for dev
};
```

## 🧪 Testing Your Setup

### Unit Test Example

```rust
#[tokio::test]
async fn test_security_system_integration() {
    let config = SecurityConfig::default();
    let security_manager = SecurityManager::new(config).unwrap();
    
    // Test authentication
    let credentials = Credentials::UsernamePassword {
        username: "test@example.com".to_string(),
        password: "password123".to_string(),
        mfa_code: None,
    };
    
    let context = security_manager.authenticate(&credentials).await.unwrap();
    assert!(context.user_id.is_some());
    
    // Test authorization
    let authorized = security_manager
        .authorize(&context, "test_resource", "read")
        .await
        .unwrap();
    assert!(authorized);
    
    // Test audit trail
    let metrics = security_manager.get_security_metrics().await.unwrap();
    assert!(metrics.authentication_events_24h >= 1);
}
```

### Integration Test with Real Database

```bash
# Setup test database
docker run --name test-postgres -e POSTGRES_PASSWORD=test -p 5432:5432 -d postgres:14
docker run --name test-redis -p 6379:6379 -d redis:7

# Run integration tests
cargo test --features integration-tests security_integration_test
```

## 📚 Next Steps

1. **[Read the Full Specification](./spec.md)** - Understand all features and requirements
2. **[Review the Technical Plan](./plan.md)** - Dive deep into implementation details  
3. **[Check Implementation Tasks](./tasks.md)** - See detailed development roadmap
4. **[API Reference](./contracts/security_compliance_api.json)** - Complete API documentation

## 🆘 Troubleshooting

### Common Issues

**Authentication Fails with "Invalid Credentials"**
- Check that JWT secret is set correctly
- Verify user exists in database
- Check password hash format

**Audit Trail Integrity Verification Fails**
- Ensure PostgreSQL has proper permissions
- Check that audit table hasn't been manually modified
- Verify cryptographic keys haven't changed

**Compliance Report Shows Low Scores**
- Review evidence collection configuration
- Check that required audit events are being logged
- Verify compliance framework mappings are correct

### Performance Optimization

**High Audit Volume**
```rust
// Enable batch processing for high-volume environments
let config = SecurityConfig {
    audit_batch_size: 10000,
    audit_compression_enabled: true,
    audit_async_processing: true,
    ..Default::default()
};
```

**Session Management at Scale**
```rust
// Configure Redis cluster for distributed sessions
let redis_config = RedisConfig {
    cluster_nodes: vec![
        "redis-node1:6379",
        "redis-node2:6379", 
        "redis-node3:6379"
    ],
    connection_pool_size: 20,
    ..Default::default()
};
```

---

**Quick Start Complete!** 🎉

You now have a fully functional Security & Compliance system with enterprise-grade features. The system provides authentication, authorization, audit trails, compliance reporting, threat detection, and privacy controls out of the box.

For production deployment, review the security configuration carefully and ensure all environment variables are set with production-appropriate values.