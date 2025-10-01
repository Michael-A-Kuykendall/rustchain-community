# RustChain Monitoring and Observability Guide

*Complete guide to monitoring, logging, and observability for RustChain deployments*

## Table of Contents
- [Overview](#overview)
- [Built-in Monitoring Features](#built-in-monitoring-features)
- [Metrics Collection](#metrics-collection)
- [Logging Configuration](#logging-configuration)
- [Health Checks](#health-checks)
- [Alerting Setup](#alerting-setup)
- [Performance Monitoring](#performance-monitoring)
- [Security Monitoring](#security-monitoring)
- [Troubleshooting](#troubleshooting)
- [Integration Guides](#integration-guides)

## Overview

RustChain includes comprehensive monitoring and observability features designed for production environments. This guide covers setup, configuration, and best practices for monitoring your RustChain deployment.

### Key Monitoring Components

- **Metrics Collection**: Prometheus-compatible metrics
- **Structured Logging**: JSON logs with tracing integration
- **Health Endpoints**: System and component health checks  
- **Audit Trails**: Cryptographic integrity tracking
- **Performance Metrics**: Execution time and resource usage
- **Security Events**: Policy violations and safety incidents

## Built-in Monitoring Features

### Metrics Endpoint

RustChain exposes Prometheus-compatible metrics on `/metrics`:

```bash
# Enable metrics in configuration
export RUSTCHAIN_METRICS_ENABLED=true
export RUSTCHAIN_METRICS_PORT=9090

# Start with metrics enabled
cargo run --bin rustchain --features observability -- serve --metrics
```

### Health Check Endpoints

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed health with component status
curl http://localhost:8080/health/detailed

# Readiness check (for Kubernetes)
curl http://localhost:8080/ready

# Liveness check (for Kubernetes)
curl http://localhost:8080/live
```

### Audit Trail Access

```bash
# View audit events
rustchain audit report --format json --since "24h ago"

# Export audit trail for analysis
rustchain audit export --output audit_trail.json --timerange "7d"

# Real-time audit monitoring
rustchain audit tail --format json | jq '.event_type'
```

## Metrics Collection

### Core Metrics

RustChain automatically collects these key metrics:

```yaml
# Mission Execution Metrics
rustchain_missions_total{status="success|failure|timeout"}
rustchain_mission_duration_seconds{mission_type="agent|chain|tool"}
rustchain_mission_steps_total{step_type="llm|command|create_file"}

# System Metrics  
rustchain_memory_usage_bytes{component="runtime|llm|memory_store"}
rustchain_cpu_usage_percent{component="executor|policy_engine"}
rustchain_disk_usage_bytes{path="/tmp|/data"}

# LLM Integration Metrics
rustchain_llm_requests_total{provider="openai|anthropic|ollama"}
rustchain_llm_response_time_seconds{provider="openai|anthropic|ollama"}
rustchain_llm_tokens_total{direction="prompt|completion"}
rustchain_llm_errors_total{provider="openai|anthropic|ollama",error_type="rate_limit|timeout|auth"}

# Security Metrics
rustchain_policy_violations_total{policy_type="safety|security|resource"}
rustchain_safety_checks_total{result="pass|fail|warn"}
rustchain_audit_events_total{event_type="mission_start|mission_complete|error"}

# Performance Metrics
rustchain_request_duration_seconds{endpoint="/api/v1/*"}
rustchain_concurrent_missions{status="running|queued"}
rustchain_tool_execution_total{tool="file_create|command|http"}
```

### Custom Metrics

Add custom metrics to your missions:

```yaml
# Mission with custom metrics
name: "Data Processing with Metrics"
steps:
  - id: "process_data"
    step_type: "tool"
    parameters:
      tool: "data_processor"
      metrics:
        - name: "records_processed"
          type: "counter"
          labels: {"dataset": "sales"}
        - name: "processing_time_seconds"  
          type: "histogram"
          buckets: [0.1, 0.5, 1.0, 5.0]
```

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'rustchain'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 5s
    metrics_path: /metrics
    
  - job_name: 'rustchain-health'
    static_configs:
      - targets: ['localhost:8080']
    scrape_interval: 30s
    metrics_path: /health/metrics
```

## Logging Configuration

### Structured Logging Setup

```toml
# rustchain.toml - Logging configuration
[logging]
level = "info"
format = "json"
output = "file"
file_path = "/var/log/rustchain/app.log"
max_file_size = "100MB"
max_files = 10

# Component-specific log levels
[logging.components]
mission_executor = "debug"
policy_engine = "warn"
llm_integration = "info"
audit_system = "info"

# Sampling for high-volume logs
[logging.sampling]
enabled = true
rate = 0.1  # Sample 10% of debug logs
```

### Environment Variable Configuration

```bash
# Set logging via environment variables
export RUST_LOG="rustchain=info,rustchain::engine=debug"
export RUSTCHAIN_LOG_FORMAT="json"
export RUSTCHAIN_LOG_FILE="/var/log/rustchain/app.log"

# Enable specific subsystem logging
export RUST_LOG="rustchain::audit=debug,rustchain::policy=warn"
```

### Log Parsing Examples

#### Using jq for JSON Logs

```bash
# Filter by log level
tail -f /var/log/rustchain/app.log | jq 'select(.level == "ERROR")'

# Mission execution logs
tail -f /var/log/rustchain/app.log | jq 'select(.target | startswith("rustchain::engine"))'

# Performance analysis
tail -f /var/log/rustchain/app.log | jq 'select(.fields.duration_ms) | {mission: .fields.mission_id, duration: .fields.duration_ms}'

# Error aggregation
tail -f /var/log/rustchain/app.log | jq -r 'select(.level == "ERROR") | .fields.error' | sort | uniq -c
```

#### Using Fluentd for Log Forwarding

```xml
<source>
  @type tail
  path /var/log/rustchain/app.log
  pos_file /var/log/td-agent/rustchain.log.pos
  tag rustchain
  format json
</source>

<match rustchain>
  @type elasticsearch
  host elasticsearch.local
  port 9200
  index_name rustchain-logs
  type_name _doc
</match>
```

## Health Checks

### Basic Health Check Response

```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime_seconds": 3600,
  "version": "1.0.0",
  "components": {
    "runtime": "healthy",
    "memory_store": "healthy", 
    "llm_providers": "healthy",
    "policy_engine": "healthy",
    "audit_system": "healthy"
  }
}
```

### Detailed Health Check Response

```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime_seconds": 3600,
  "version": "1.0.0",
  "components": {
    "runtime": {
      "status": "healthy",
      "memory_usage_mb": 45.2,
      "cpu_usage_percent": 12.5,
      "active_missions": 3
    },
    "memory_store": {
      "status": "healthy",
      "total_entries": 150,
      "memory_usage_mb": 8.3,
      "hit_rate_percent": 85.2
    },
    "llm_providers": {
      "status": "healthy",
      "active_providers": ["openai", "anthropic"],
      "total_requests": 1247,
      "avg_response_time_ms": 850
    },
    "policy_engine": {
      "status": "healthy",
      "policies_loaded": 12,
      "violations_last_hour": 0
    },
    "audit_system": {
      "status": "healthy",
      "events_last_hour": 45,
      "storage_usage_mb": 23.1
    }
  }
}
```

### Custom Health Checks

```rust
// Add custom health checks
use rustchain::health::{HealthCheck, HealthStatus};

struct CustomDatabaseCheck {
    db_pool: DatabasePool,
}

impl HealthCheck for CustomDatabaseCheck {
    async fn check(&self) -> HealthStatus {
        match self.db_pool.get().await {
            Ok(_) => HealthStatus::Healthy,
            Err(e) => HealthStatus::Unhealthy {
                message: format!("Database connection failed: {}", e)
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "database"
    }
}
```

## Alerting Setup

### Prometheus AlertManager Rules

```yaml
# rustchain-alerts.yml
groups:
  - name: rustchain
    rules:
      # High error rate
      - alert: HighMissionFailureRate
        expr: rate(rustchain_missions_total{status="failure"}[5m]) > 0.1
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "High mission failure rate detected"
          description: "Mission failure rate is {{ $value }} per second"
          
      # Memory usage
      - alert: HighMemoryUsage
        expr: rustchain_memory_usage_bytes / (1024*1024*1024) > 2
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage"
          description: "Memory usage is {{ $value }}GB"
          
      # LLM provider issues
      - alert: LLMProviderDown
        expr: up{job="rustchain"} == 0 or rate(rustchain_llm_errors_total[5m]) > 0.5
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "LLM provider issues detected"
          description: "LLM provider {{ $labels.provider }} is experiencing issues"
          
      # Policy violations
      - alert: PolicyViolations
        expr: increase(rustchain_policy_violations_total[15m]) > 5
        for: 0s
        labels:
          severity: critical
        annotations:
          summary: "Multiple policy violations detected"
          description: "{{ $value }} policy violations in the last 15 minutes"
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "RustChain Monitoring",
    "panels": [
      {
        "title": "Mission Success Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "rate(rustchain_missions_total{status=\"success\"}[5m]) / rate(rustchain_missions_total[5m])",
            "legendFormat": "Success Rate"
          }
        ]
      },
      {
        "title": "Mission Duration",
        "type": "graph", 
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rustchain_mission_duration_seconds_bucket)",
            "legendFormat": "95th percentile"
          },
          {
            "expr": "histogram_quantile(0.5, rustchain_mission_duration_seconds_bucket)",
            "legendFormat": "Median"
          }
        ]
      },
      {
        "title": "System Resources",
        "type": "graph",
        "targets": [
          {
            "expr": "rustchain_memory_usage_bytes / (1024*1024)",
            "legendFormat": "Memory Usage (MB)"
          },
          {
            "expr": "rustchain_cpu_usage_percent",
            "legendFormat": "CPU Usage %"
          }
        ]
      }
    ]
  }
}
```

## Performance Monitoring

### Response Time Analysis

```bash
# Mission execution times by type
curl -s http://localhost:9090/metrics | grep rustchain_mission_duration | \
  awk -F'[{}]' '{print $2 " " $3}' | sort -k2 -n

# LLM provider performance comparison
curl -s http://localhost:9090/metrics | grep rustchain_llm_response_time | \
  awk -F'[{}]' '{print $2 " " $3}' | sort -k2 -n
```

### Resource Usage Tracking

```yaml
# Mission with resource monitoring
name: "Resource Intensive Task"
config:
  resource_limits:
    max_memory_mb: 500
    max_cpu_percent: 80
    max_duration_seconds: 300
  monitoring:
    track_memory: true
    track_cpu: true
    sample_interval_seconds: 5
    
steps:
  - id: "heavy_computation"
    step_type: "tool"
    parameters:
      tool: "data_processor"
      resource_profile: "memory_intensive"
```

### Performance Baselines

```bash
# Establish performance baselines
rustchain benchmark --output baseline.json --iterations 100

# Compare current performance to baseline
rustchain benchmark --baseline baseline.json --threshold 0.2

# Continuous performance testing
rustchain benchmark --continuous --alert-threshold 0.3
```

## Security Monitoring

### Audit Event Analysis

```bash
# Real-time security monitoring
rustchain audit tail --filter "event_type=policy_violation" | \
  jq '{time: .timestamp, policy: .policy_name, violation: .violation_type}'

# Daily security report
rustchain audit report --format json --since "1d ago" | \
  jq -r '.events[] | select(.event_type == "policy_violation") | 
         [.timestamp, .policy_name, .violation_type] | @csv'
```

### Security Metrics Dashboard

```yaml
# Security-focused metrics
rustchain_security_events_total{event_type="policy_violation|safety_check|auth_failure"}
rustchain_failed_authentications_total{source="api|cli|webhook"}
rustchain_suspicious_activities_total{activity_type="unusual_pattern|rate_limit_exceeded"}
```

## Troubleshooting

### Common Monitoring Issues

#### Metrics Not Appearing

```bash
# Check metrics endpoint
curl -v http://localhost:9090/metrics

# Verify feature compilation
cargo build --features observability

# Check configuration
rustchain config show | grep -i metrics
```

#### High Memory Usage

```bash
# Analyze memory usage by component
curl -s http://localhost:9090/metrics | grep rustchain_memory_usage_bytes

# Check for memory leaks in missions
rustchain audit report --filter "event_type=memory_warning" --since "1h ago"

# Optimize memory store configuration
export RUSTCHAIN_MEMORY_STORE_TTL=3600
export RUSTCHAIN_MEMORY_STORE_MAX_SIZE=1000
```

#### Slow Mission Execution

```bash
# Identify slow missions
rustchain audit report --format json --since "1h ago" | \
  jq -r '.events[] | select(.event_type == "mission_complete") | 
         [.mission_id, .duration_ms] | @csv' | sort -t, -k2 -n

# LLM provider performance analysis
curl -s http://localhost:9090/metrics | grep rustchain_llm_response_time
```

### Log Analysis Scripts

```bash
#!/bin/bash
# analyze_logs.sh - Comprehensive log analysis

LOG_FILE=${1:-/var/log/rustchain/app.log}

echo "=== RustChain Log Analysis ==="
echo "Log file: $LOG_FILE"
echo "Analysis time: $(date)"
echo

# Error summary
echo "Top 10 Errors:"
grep '"level":"ERROR"' "$LOG_FILE" | \
  jq -r '.fields.error' | sort | uniq -c | sort -nr | head -10

# Mission performance
echo -e "\nMission Performance (last 1000 entries):"
tail -n 1000 "$LOG_FILE" | grep '"target":"rustchain::engine"' | \
  jq -r 'select(.fields.duration_ms) | [.fields.mission_id, .fields.duration_ms] | @csv' | \
  awk -F, '{sum+=$2; count++} END {print "Average duration:", sum/count "ms"}'

# Policy violations
echo -e "\nPolicy Violations:"
grep '"event_type":"policy_violation"' "$LOG_FILE" | \
  jq -r '[.timestamp, .policy_name, .violation_type] | @csv' | tail -5
```

## Integration Guides

### ELK Stack Integration

#### Logstash Configuration

```ruby
input {
  file {
    path => "/var/log/rustchain/app.log"
    codec => json
    tags => ["rustchain"]
  }
}

filter {
  if "rustchain" in [tags] {
    mutate {
      add_field => { "service" => "rustchain" }
    }
    
    if [level] == "ERROR" {
      mutate {
        add_tag => ["error"]
      }
    }
  }
}

output {
  elasticsearch {
    hosts => ["elasticsearch:9200"]
    index => "rustchain-logs-%{+YYYY.MM.dd}"
  }
}
```

### Datadog Integration

```yaml
# datadog-rustchain.yaml
logs:
  - type: file
    path: /var/log/rustchain/app.log
    service: rustchain
    source: rust
    tags:
      - env:production
      - team:ai-platform

# Custom metrics
init_config:

instances:
  - prometheus_url: http://localhost:9090/metrics
    namespace: rustchain
    metrics:
      - rustchain_missions_total
      - rustchain_mission_duration_seconds
      - rustchain_memory_usage_bytes
```

### New Relic Integration

```toml
# newrelic.toml
[newrelic]
license_key = "your-license-key"
app_name = "RustChain"

[newrelic.distributed_tracing]
enabled = true

[newrelic.attributes]
enabled = true
include = ["request.*", "mission.*"]
```

### Kubernetes Monitoring

```yaml
# ServiceMonitor for Prometheus Operator
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: rustchain
spec:
  selector:
    matchLabels:
      app: rustchain
  endpoints:
  - port: metrics
    interval: 15s
    path: /metrics
    
---
# PodMonitor for detailed metrics
apiVersion: monitoring.coreos.com/v1
kind: PodMonitor
metadata:
  name: rustchain-pods
spec:
  selector:
    matchLabels:
      app: rustchain
  podMetricsEndpoints:
  - port: metrics
    interval: 15s
```

### Sample Grafana Queries

```promql
# Mission success rate over time
rate(rustchain_missions_total{status="success"}[5m]) / rate(rustchain_missions_total[5m])

# 95th percentile mission duration
histogram_quantile(0.95, rate(rustchain_mission_duration_seconds_bucket[5m]))

# Memory usage trend
rustchain_memory_usage_bytes / 1024 / 1024

# Error rate by component
rate(rustchain_errors_total[5m]) by (component)

# LLM provider comparison
avg_over_time(rustchain_llm_response_time_seconds[1h]) by (provider)
```

## Conclusion

This monitoring and observability setup provides comprehensive visibility into your RustChain deployment. Key benefits:

- **Proactive Issue Detection**: Catch problems before they impact users
- **Performance Optimization**: Identify bottlenecks and optimization opportunities  
- **Security Monitoring**: Track policy violations and security events
- **Operational Insights**: Understand usage patterns and system behavior
- **Compliance**: Maintain audit trails and meet regulatory requirements

For advanced monitoring scenarios or custom integrations, consult the [RustChain API Reference](API_REFERENCE.md) and consider the Enterprise Edition for additional monitoring features.