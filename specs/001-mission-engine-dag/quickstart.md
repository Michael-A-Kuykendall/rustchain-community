# Quickstart: Mission Engine Validation

**Generated**: 2025-01-20  
**Purpose**: 5-minute validation of mission engine functionality  
**Context**: Verify core DAG execution, audit trails, and policy enforcement

## Prerequisites

1. **RustChain installed** with mission engine features
2. **CLI access** to rustchain commands
3. **Write permissions** in test directory
4. **Basic YAML knowledge** for mission definition

## Quick Validation Scenario

### Step 1: Create Test Mission (1 minute)

Create a simple 3-step mission to validate core functionality:

```bash
# Create test mission file
cat > test_mission_engine.yaml << 'EOF'
version: "1.0"
name: "Mission Engine Validation"
description: "Quick validation of DAG execution with dependencies"

config:
  timeout_seconds: 30
  fail_fast: true
  parallel_limit: 2

steps:
  - id: "setup"
    name: "Create test file"
    step_type: "create_file"
    parameters:
      path: "./mission_test_output.txt"
      content: "Mission Engine Test Started\nTimestamp: $(date)\n"
    
  - id: "process"
    name: "Process test data"
    step_type: "command"
    depends_on: ["setup"]
    parameters:
      command: "echo"
      args: ["Processing complete - Mission Engine Working"]
    
  - id: "verify"
    name: "Verify and append result"
    step_type: "create_file"
    depends_on: ["process"]
    parameters:
      path: "./mission_test_result.txt"
      content: "Mission Engine Validation: SUCCESS\nAll dependencies resolved correctly\nDAG execution order verified\n"
      
  - id: "cleanup"
    name: "Log completion"
    step_type: "command"
    depends_on: ["verify"]
    parameters:
      command: "echo"
      args: ["Mission Engine validation complete"]
EOF
```

### Step 2: Execute Mission (2 minutes)

Execute the mission and verify DAG-based execution:

```bash
# Execute the test mission
rustchain mission execute test_mission_engine.yaml

# Expected output should show:
# 1. Mission validation success
# 2. Steps executed in dependency order: setup → process → verify → cleanup
# 3. All steps completed successfully
# 4. Total execution time reported
```

**Expected Execution Order**:
```
[INFO] Executing mission 'Mission Engine Validation' with 4 steps
[DEBUG] Executing step: setup (Create test file)
[DEBUG] Executing step: process (Process test data)
[DEBUG] Executing step: verify (Verify and append result)
[DEBUG] Executing step: cleanup (Log completion)
[INFO] Mission completed successfully in ~2-5 seconds
```

### Step 3: Verify Audit Trail (1 minute)

Validate that comprehensive audit logging is working:

```bash
# Check audit trail for the mission
rustchain audit report --mission "Mission Engine Validation"

# Expected output should include:
# - Mission start event with timestamp
# - Each step execution start/complete events
# - Final mission completion event
# - Cryptographic integrity verification
```

**Expected Audit Output**:
```json
{
  "mission_name": "Mission Engine Validation",
  "events": [
    {
      "event_type": "mission_start",
      "timestamp": "2025-01-20T...",
      "actor": "user",
      "target": "test_mission_engine.yaml"
    },
    {
      "event_type": "step_complete",
      "step_id": "setup",
      "status": "Completed",
      "duration_ms": "..."
    }
  ],
  "integrity_verified": true,
  "total_events": 9
}
```

### Step 4: Validate Policy Enforcement (1 minute)

Test that security policies are enforced:

```bash
# Create a mission with potential security issues
cat > test_security_mission.yaml << 'EOF'
version: "1.0"
name: "Security Test Mission"
steps:
  - id: "dangerous_command"
    name: "Test command injection prevention"
    step_type: "command"
    parameters:
      command: "echo"
      args: ["test && rm -rf /"]
EOF

# Execute and expect policy violation
rustchain mission execute test_security_mission.yaml

# Expected: Execution blocked with security policy violation
```

**Expected Security Response**:
```
[ERROR] Policy violation detected in step 'dangerous_command'
[ERROR] Dangerous command pattern detected: rm -rf
[ERROR] Mission execution blocked for security
[INFO] Audit event logged: policy_violation
```

### Step 5: Verify Generated Files (30 seconds)

Check that the mission actually created the expected outputs:

```bash
# Verify test files were created
ls -la mission_test_*.txt

# Check file contents
cat mission_test_output.txt
cat mission_test_result.txt

# Expected: Both files should exist with correct content
```

## Validation Checklist

After completing the quickstart, verify these capabilities:

### Core Functionality ✅
- [ ] Mission definition parsing works correctly
- [ ] DAG dependency resolution executes steps in correct order
- [ ] All 4 test steps completed successfully
- [ ] File creation and command execution step types work
- [ ] Mission completes with success status

### Security & Compliance ✅
- [ ] Dangerous commands are blocked by security policies
- [ ] Policy violations are logged in audit trail
- [ ] Input sanitization prevents command injection
- [ ] File path validation prevents directory traversal

### Audit & Monitoring ✅
- [ ] All mission events are logged with timestamps
- [ ] Step-level execution details are captured
- [ ] Audit trail integrity is cryptographically verified
- [ ] Audit reports are accessible via CLI

### Error Handling ✅
- [ ] Mission validation catches invalid definitions
- [ ] Policy violations stop execution immediately
- [ ] Timeout handling works for long-running steps
- [ ] Error messages are clear and actionable

### Performance ✅
- [ ] Simple 4-step mission completes in <10 seconds
- [ ] Step execution overhead is minimal
- [ ] Memory usage remains reasonable during execution
- [ ] No resource leaks or excessive CPU usage

## Troubleshooting

### Common Issues

**Mission won't execute**:
- Check YAML syntax with `rustchain mission validate test_mission_engine.yaml`
- Verify file permissions in current directory
- Ensure rustchain CLI is properly installed

**Steps execute out of order**:
- Verify `depends_on` dependencies are correctly specified
- Check for circular dependencies in mission definition
- Review debug logs for topological sort output

**Security policies too restrictive**:
- Review policy configuration with `rustchain policy list`
- Customize policies if needed for specific environments
- Check audit logs for specific policy violations

**Audit trail missing**:
- Verify audit logging is enabled in configuration
- Check file system permissions for audit log directory
- Ensure cryptographic dependencies are available

## Success Criteria

✅ **Mission Engine Validated** when:
- All 4 test steps execute in correct dependency order
- Security policies block dangerous operations
- Comprehensive audit trail is generated and verified
- Files are created with expected content
- No errors or warnings in execution logs

## Next Steps

After successful validation:

1. **Explore Advanced Features**: Try more complex missions with parallel execution
2. **Review Documentation**: Read full mission engine specification
3. **Create Production Missions**: Design workflows for your specific use cases
4. **Monitor Performance**: Set up monitoring for production deployments
5. **Customize Policies**: Configure security policies for your environment

---
**Quickstart Complete**: Mission Engine core functionality validated in 5 minutes