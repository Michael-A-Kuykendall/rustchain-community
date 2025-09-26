# 🚀 RustChain Production Deployment Guide

*Your complete guide to deploying RustChain in production environments*

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
- [Configuration Management](#configuration-management)
- [Security Hardening](#security-hardening)
- [Performance Optimization](#performance-optimization)
- [Monitoring & Observability](#monitoring--observability)
- [High Availability Setup](#high-availability-setup)
- [Backup & Recovery](#backup--recovery)
- [Troubleshooting](#troubleshooting)
- [Maintenance](#maintenance)

## 🔧 Prerequisites

### System Requirements

**Minimum Requirements:**
- **CPU**: 2 cores, 2.0 GHz
- **RAM**: 4 GB
- **Disk**: 20 GB available space
- **Network**: Stable internet connection for LLM providers

**Recommended Production:**
- **CPU**: 4+ cores, 3.0+ GHz
- **RAM**: 16+ GB
- **Disk**: 100+ GB SSD storage
- **Network**: High-bandwidth connection, low latency to LLM providers

### Software Dependencies

```bash
# Rust toolchain (required)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# System packages (Ubuntu/Debian)
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev sqlite3

# System packages (CentOS/RHEL)
sudo yum groupinstall -y "Development Tools"
sudo yum install -y openssl-devel sqlite

# System packages (macOS)
brew install openssl sqlite
```

## 📦 Installation Methods

### Method 1: Binary Release (Recommended)

```bash
# Download latest release
curl -L -o rustchain https://github.com/rustchain-community/rustchain-community/releases/latest/download/rustchain-linux-x64

# Make executable and install
chmod +x rustchain
sudo mv rustchain /usr/local/bin/

# Verify installation
rustchain --version
```

### Method 2: Build from Source

```bash
# Clone repository
git clone https://github.com/rustchain-community/rustchain-community.git
cd rustchain-community

# Build optimized release
cargo build --release --all-features

# Install binary
sudo cp target/release/rustchain /usr/local/bin/
```

### Method 3: Docker Deployment

```dockerfile
# Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --all-features

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rustchain /usr/local/bin/
EXPOSE 8080
CMD ["rustchain", "server", "start"]
```

```bash
# Build and run
docker build -t rustchain:latest .
docker run -d -p 8080:8080 --name rustchain-server rustchain:latest
```

## ⚙️ Configuration Management

### Production Configuration Structure

```yaml
# /etc/rustchain/config.yaml
server:
  host: "0.0.0.0"
  port: 8080
  workers: 4
  max_connections: 1000

security:
  enable_auth: true
  jwt_secret_file: "/etc/rustchain/secrets/jwt.key"
  api_rate_limit: 1000  # requests per minute
  
llm:
  providers:
    openai:
      api_key_file: "/etc/rustchain/secrets/openai.key"
      model: "gpt-4"
      timeout: 30
    anthropic:
      api_key_file: "/etc/rustchain/secrets/anthropic.key" 
      model: "claude-3-sonnet"
      timeout: 30

storage:
  type: "sqlite"
  database_path: "/var/lib/rustchain/data.db"
  backup_path: "/var/lib/rustchain/backups"
  retention_days: 30

memory:
  store_type: "sqlite"
  capacity_mb: 1024
  ttl_hours: 24

audit:
  enabled: true
  log_path: "/var/log/rustchain/audit.log"
  rotate_daily: true
  retention_days: 90

policy:
  config_path: "/etc/rustchain/policies"
  enforce_safety: true
  sandbox_enabled: true

tools:
  execution_timeout: 300
  allowed_commands: 
    - "echo"
    - "ls"
    - "cat"
  blocked_patterns:
    - "rm -rf"
    - "sudo"
    - "chmod 777"
```

### Environment-Specific Configurations

```bash
# Production environment variables
export RUSTCHAIN_ENV=production
export RUSTCHAIN_CONFIG=/etc/rustchain/config.yaml
export RUSTCHAIN_LOG_LEVEL=info
export RUST_LOG=rustchain=info,warn
export RUST_BACKTRACE=0  # Disable in production
```

### Secrets Management

```bash
# Create secure directories
sudo mkdir -p /etc/rustchain/secrets
sudo chmod 700 /etc/rustchain/secrets

# Store API keys securely
sudo tee /etc/rustchain/secrets/openai.key << EOF
sk-your-openai-api-key-here
EOF

sudo tee /etc/rustchain/secrets/anthropic.key << EOF
your-anthropic-api-key-here
EOF

# Generate JWT secret
openssl rand -base64 32 | sudo tee /etc/rustchain/secrets/jwt.key

# Secure permissions
sudo chmod 600 /etc/rustchain/secrets/*
sudo chown rustchain:rustchain /etc/rustchain/secrets/*
```

## 🔒 Security Hardening

### User & Permissions Setup

```bash
# Create dedicated user
sudo useradd --system --shell /bin/false --home /var/lib/rustchain rustchain

# Create directories with proper permissions
sudo mkdir -p /var/lib/rustchain/{data,backups,temp}
sudo mkdir -p /var/log/rustchain
sudo chown -R rustchain:rustchain /var/lib/rustchain /var/log/rustchain
sudo chmod 750 /var/lib/rustchain /var/log/rustchain
```

### Firewall Configuration

```bash
# UFW (Ubuntu)
sudo ufw allow from 10.0.0.0/8 to any port 8080  # Internal only
sudo ufw deny 8080  # Block external access
sudo ufw reload

# iptables
sudo iptables -A INPUT -p tcp --dport 8080 -s 10.0.0.0/8 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8080 -j DROP
```

### TLS/SSL Setup

```bash
# Generate self-signed certificate (development)
openssl req -x509 -newkey rsa:4096 -keyout /etc/rustchain/server.key -out /etc/rustchain/server.crt -days 365 -nodes

# Or use Let's Encrypt (production)
sudo certbot certonly --standalone -d your-domain.com
sudo ln -sf /etc/letsencrypt/live/your-domain.com/fullchain.pem /etc/rustchain/server.crt
sudo ln -sf /etc/letsencrypt/live/your-domain.com/privkey.pem /etc/rustchain/server.key
```

### Security Policy Configuration

```yaml
# /etc/rustchain/policies/security.yaml
sandbox:
  enabled: true
  max_execution_time: 300
  max_memory_mb: 512
  network_access: false
  file_system_access: "restricted"
  
validation:
  max_mission_size_mb: 10
  max_steps_per_mission: 100
  allowed_step_types:
    - "llm"
    - "create_file"
    - "http"
  blocked_patterns:
    - "eval"
    - "exec"
    - "system"

rate_limiting:
  per_user_per_minute: 60
  per_ip_per_minute: 100
  burst_limit: 10
```

## ⚡ Performance Optimization

### Rust Compiler Optimizations

```bash
# Build with maximum optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release --all-features

# Profile-guided optimization
cargo build --release --all-features
# Run typical workload to generate profile data
cargo pgo build
```

### Runtime Performance Tuning

```yaml
# Performance-optimized configuration
server:
  workers: 8  # CPU cores * 2
  max_connections: 2000
  keepalive_timeout: 60
  request_timeout: 300

memory:
  store_type: "memory"  # Faster than SQLite for cache
  capacity_mb: 4096
  cleanup_interval: 3600

llm:
  connection_pool_size: 20
  request_timeout: 45
  retry_attempts: 2
  
storage:
  connection_pool_size: 10
  statement_cache_size: 1000
  journal_mode: "WAL"  # Better SQLite performance
```

### System-Level Optimizations

```bash
# Increase file descriptor limits
echo "rustchain soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "rustchain hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# TCP optimization
sudo sysctl -w net.core.somaxconn=65535
sudo sysctl -w net.ipv4.tcp_max_syn_backlog=65535
sudo sysctl -w net.core.netdev_max_backlog=5000

# Make persistent
echo "net.core.somaxconn=65535" | sudo tee -a /etc/sysctl.conf
```

## 📊 Monitoring & Observability

### Systemd Service Setup

```ini
# /etc/systemd/system/rustchain.service
[Unit]
Description=RustChain AI Orchestration Server
After=network.target
Requires=network.target

[Service]
Type=simple
User=rustchain
Group=rustchain
WorkingDirectory=/var/lib/rustchain
ExecStart=/usr/local/bin/rustchain server start
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
Environment=RUSTCHAIN_CONFIG=/etc/rustchain/config.yaml
Environment=RUSTCHAIN_ENV=production

# Security hardening
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ProtectKernelTunables=yes
ProtectControlGroups=yes
ReadWritePaths=/var/lib/rustchain /var/log/rustchain

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable rustchain
sudo systemctl start rustchain
sudo systemctl status rustchain
```

### Log Management

```bash
# Configure logrotate
sudo tee /etc/logrotate.d/rustchain << EOF
/var/log/rustchain/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 rustchain rustchain
    postrotate
        systemctl reload rustchain
    endscript
}
EOF
```

### Health Check Endpoints

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed status
curl http://localhost:8080/status

# Metrics (if enabled)
curl http://localhost:8080/metrics
```

### Monitoring Integration

```yaml
# Prometheus scrape configuration
scrape_configs:
  - job_name: 'rustchain'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

## 🏗️ High Availability Setup

### Load Balancer Configuration (Nginx)

```nginx
# /etc/nginx/sites-available/rustchain
upstream rustchain_backend {
    server 10.0.1.10:8080 weight=1 max_fails=3 fail_timeout=30s;
    server 10.0.1.11:8080 weight=1 max_fails=3 fail_timeout=30s;
    server 10.0.1.12:8080 weight=1 max_fails=3 fail_timeout=30s;
    keepalive 32;
}

server {
    listen 443 ssl http2;
    server_name rustchain.yourdomain.com;
    
    ssl_certificate /etc/ssl/certs/rustchain.crt;
    ssl_certificate_key /etc/ssl/private/rustchain.key;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    
    location / {
        proxy_pass http://rustchain_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
        proxy_connect_timeout 5s;
        proxy_send_timeout 300s;
        proxy_read_timeout 300s;
    }
    
    location /health {
        proxy_pass http://rustchain_backend/health;
        access_log off;
    }
}
```

### Database Replication (PostgreSQL)

```yaml
# For PostgreSQL backend (enterprise edition)
storage:
  type: "postgresql"
  primary:
    host: "db-primary.internal"
    port: 5432
    database: "rustchain"
    user: "rustchain"
    password_file: "/etc/rustchain/secrets/db.pass"
  replicas:
    - host: "db-replica-1.internal"
    - host: "db-replica-2.internal"
  pool_size: 20
  connection_timeout: 10
```

## 💾 Backup & Recovery

### Automated Backup Strategy

```bash
#!/bin/bash
# /usr/local/bin/rustchain-backup.sh

BACKUP_DIR="/var/lib/rustchain/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="rustchain_backup_${TIMESTAMP}"

# Create backup directory
mkdir -p "${BACKUP_DIR}/${BACKUP_NAME}"

# Backup database
sqlite3 /var/lib/rustchain/data.db ".backup ${BACKUP_DIR}/${BACKUP_NAME}/data.db"

# Backup configuration
cp -r /etc/rustchain "${BACKUP_DIR}/${BACKUP_NAME}/config"

# Backup logs (last 7 days)
find /var/log/rustchain -name "*.log" -mtime -7 -exec cp {} "${BACKUP_DIR}/${BACKUP_NAME}/" \;

# Compress backup
tar -czf "${BACKUP_DIR}/${BACKUP_NAME}.tar.gz" -C "${BACKUP_DIR}" "${BACKUP_NAME}"
rm -rf "${BACKUP_DIR}/${BACKUP_NAME}"

# Cleanup old backups (keep 30 days)
find "${BACKUP_DIR}" -name "rustchain_backup_*.tar.gz" -mtime +30 -delete

echo "Backup completed: ${BACKUP_DIR}/${BACKUP_NAME}.tar.gz"
```

```bash
# Add to crontab
sudo crontab -u rustchain -e
# Add line: 0 2 * * * /usr/local/bin/rustchain-backup.sh
```

### Recovery Procedures

```bash
# Stop service
sudo systemctl stop rustchain

# Restore from backup
BACKUP_FILE="rustchain_backup_20240101_020000.tar.gz"
tar -xzf "/var/lib/rustchain/backups/${BACKUP_FILE}" -C /tmp/

# Restore database
cp /tmp/rustchain_backup_*/data.db /var/lib/rustchain/data.db

# Restore configuration
sudo cp -r /tmp/rustchain_backup_*/config/* /etc/rustchain/

# Fix permissions
sudo chown -R rustchain:rustchain /var/lib/rustchain
sudo chown -R root:root /etc/rustchain
sudo chmod 600 /etc/rustchain/secrets/*

# Start service
sudo systemctl start rustchain
```

## 🔍 Troubleshooting

### Common Issues & Solutions

#### Service Won't Start

```bash
# Check service status
sudo systemctl status rustchain

# Check logs
sudo journalctl -u rustchain -f

# Verify configuration
rustchain config validate /etc/rustchain/config.yaml

# Check permissions
ls -la /etc/rustchain/secrets/
ls -la /var/lib/rustchain/
```

#### High Memory Usage

```bash
# Check memory usage
ps aux | grep rustchain
sudo systemctl show rustchain --property=MemoryCurrent

# Adjust memory limits in configuration
# memory.capacity_mb: 512  # Reduce if needed
```

#### Connection Timeouts

```bash
# Check network connectivity
curl -v http://localhost:8080/health

# Verify LLM provider connectivity
curl -H "Authorization: Bearer $OPENAI_API_KEY" https://api.openai.com/v1/models

# Check firewall rules
sudo ufw status
sudo iptables -L
```

#### Database Corruption

```bash
# Check database integrity
sqlite3 /var/lib/rustchain/data.db "PRAGMA integrity_check;"

# Repair if needed
sqlite3 /var/lib/rustchain/data.db "VACUUM;"

# Restore from backup if severely corrupted
```

### Debug Mode

```yaml
# Enable debug logging temporarily
logging:
  level: "debug"
  rust_log: "rustchain=debug,trace"
  
# Or via environment
export RUST_LOG=rustchain=debug,trace
export RUSTCHAIN_LOG_LEVEL=debug
```

## 🔄 Maintenance

### Regular Maintenance Tasks

```bash
#!/bin/bash
# /usr/local/bin/rustchain-maintenance.sh

# Database optimization
echo "Optimizing database..."
sqlite3 /var/lib/rustchain/data.db "VACUUM; ANALYZE;"

# Log rotation check
echo "Checking log rotation..."
logrotate -f /etc/logrotate.d/rustchain

# Update system packages
echo "Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Check disk space
echo "Checking disk space..."
df -h /var/lib/rustchain
df -h /var/log/rustchain

# Restart service monthly
if [ $(date +%d) -eq 01 ]; then
    echo "Monthly service restart..."
    sudo systemctl restart rustchain
fi

echo "Maintenance completed at $(date)"
```

### Performance Monitoring

```bash
# Create monitoring script
#!/bin/bash
# /usr/local/bin/rustchain-monitor.sh

# Check service health
if ! curl -sf http://localhost:8080/health > /dev/null; then
    echo "ALERT: RustChain health check failed"
    sudo systemctl restart rustchain
fi

# Monitor resource usage
MEM_USAGE=$(ps -o pid,ppid,cmd,%mem --sort=-%mem | grep rustchain | head -1 | awk '{print $4}')
if (( $(echo "$MEM_USAGE > 80" | bc -l) )); then
    echo "WARNING: High memory usage: ${MEM_USAGE}%"
fi

# Check log errors
ERROR_COUNT=$(grep -c "ERROR\|CRITICAL" /var/log/rustchain/*.log)
if [ "$ERROR_COUNT" -gt 10 ]; then
    echo "WARNING: ${ERROR_COUNT} errors found in logs"
fi
```

### Update Procedures

```bash
# Backup before update
/usr/local/bin/rustchain-backup.sh

# Download new version
curl -L -o rustchain-new https://github.com/rustchain-community/rustchain-community/releases/latest/download/rustchain-linux-x64

# Test new version
chmod +x rustchain-new
./rustchain-new --version
./rustchain-new config validate /etc/rustchain/config.yaml

# Deploy update
sudo systemctl stop rustchain
sudo cp rustchain-new /usr/local/bin/rustchain
sudo systemctl start rustchain

# Verify update
rustchain --version
curl http://localhost:8080/health
```

## 📞 Support & Resources

### Getting Help

- **Documentation**: [GitHub Wiki](https://github.com/rustchain-community/rustchain-community/wiki)
- **Issues**: [GitHub Issues](https://github.com/rustchain-community/rustchain-community/issues)
- **Discussions**: [GitHub Discussions](https://github.com/rustchain-community/rustchain-community/discussions)
- **Security Issues**: security@rustchain.dev

### Performance Baselines

| Metric | Minimum | Good | Excellent |
|--------|---------|------|-----------|
| Response Time | < 5s | < 2s | < 1s |
| Throughput | 10 req/s | 50 req/s | 100+ req/s |
| Memory Usage | < 2GB | < 1GB | < 512MB |
| CPU Usage | < 80% | < 50% | < 25% |
| Uptime | 99% | 99.9% | 99.99% |

---

*This deployment guide covers production-ready deployment scenarios. For development setup, see [CONTRIBUTING.md](../CONTRIBUTING.md). For API usage, see [API_REFERENCE.md](API_REFERENCE.md).*