# RustChain Examples

This directory contains example missions that demonstrate various RustChain features and capabilities.

## Available Examples

### 1. Hello World (`hello_world.json`)
**Difficulty**: Beginner  
**Duration**: ~1 minute  
**Description**: A simple introductory mission that demonstrates basic RustChain functionality.

**Features Demonstrated**:
- Basic mission structure
- Sequential step execution
- Command-line tool integration
- Dependency resolution

**Usage**:
```bash
rustchain mission create --file examples/hello_world.json
rustchain mission execute --id hello_world_001
```

### 2. Data Processing Pipeline (`data_processing.json`)
**Difficulty**: Intermediate  
**Duration**: ~3 minutes  
**Description**: A comprehensive data processing mission that demonstrates file operations, data validation, and analysis.

**Features Demonstrated**:
- File system operations
- Data generation and analysis
- Python integration
- Error handling and cleanup
- Complex dependency chains

**Requirements**: Python 3, bash, standard Unix tools

**Usage**:
```bash
rustchain mission create --file examples/data_processing.json
rustchain mission execute --id data_processing_001
```

### 3. Safety Validation Demo (`safety_demo.json`)
**Difficulty**: Beginner  
**Duration**: ~1 minute  
**Description**: Demonstrates safety validation features by performing safe operations.

**Features Demonstrated**:
- Safe file operations
- Safety validator integration
- Temporary file handling
- Proper cleanup procedures

**Usage**:
```bash
rustchain mission create --file examples/safety_demo.json
rustchain mission execute --id safety_demo_001
```

## Running Examples

### Prerequisites

1. **RustChain Installation**: Ensure RustChain is installed and configured
2. **Dependencies**: Install required tools (Python, bash, etc.)
3. **Permissions**: Ensure appropriate file system permissions

### Basic Execution

1. **Load Mission**:
   ```bash
   rustchain mission create --file examples/[example_name].json
   ```

2. **Validate Mission** (optional):
   ```bash
   rustchain policy validate --mission-file examples/[example_name].json
   ```

3. **Execute Mission**:
   ```bash
   rustchain mission execute --id [mission_id]
   ```

4. **Monitor Progress**:
   ```bash
   rustchain mission status --id [mission_id]
   rustchain mission logs --id [mission_id] --follow
   ```

### Advanced Usage

#### Dry Run
Test missions without actual execution:
```bash
rustchain mission execute --id [mission_id] --dry-run
```

#### Skip Safety Validation
Skip safety checks (use with caution):
```bash
rustchain mission execute --id [mission_id] --skip-safety
```

#### Custom Configuration
Use custom configuration file:
```bash
rustchain --config custom.toml mission execute --id [mission_id]
```

## Creating Custom Examples

### Mission Structure

```json
{
  "id": "unique_mission_id",
  "name": "Human Readable Name",
  "description": "Detailed description of the mission",
  "version": "1.0.0",
  "steps": [
    {
      "id": "step_id",
      "name": "Step Name",
      "step_type": "Tool",
      "config": {
        "tool": "command_name",
        "args": ["arg1", "arg2"]
      },
      "dependencies": ["previous_step_id"],
      "timeout": 30,
      "retry_count": 1,
      "on_failure": "continue"
    }
  ],
  "metadata": {
    "category": "example_category",
    "difficulty": "beginner|intermediate|advanced",
    "estimated_duration": "X minutes"
  }
}
```

### Best Practices

1. **Use Descriptive IDs**: Make step IDs and mission IDs descriptive
2. **Set Appropriate Timeouts**: Consider the expected execution time
3. **Handle Failures**: Use appropriate failure handling strategies
4. **Add Dependencies**: Ensure proper step ordering
5. **Include Metadata**: Provide helpful categorization and timing info
6. **Test Thoroughly**: Validate missions before sharing

### Step Types

- **Tool**: Execute command-line tools
- **LLM**: Interact with language models (requires LLM feature)
- **RAG**: Perform document retrieval (requires RAG feature)
- **Conditional**: Execute based on conditions
- **Parallel**: Execute multiple steps concurrently

### Safety Considerations

- Always use safe file paths (prefer `/tmp` for temporary files)
- Avoid destructive operations in examples
- Include proper cleanup steps
- Test in isolated environments first
- Use appropriate retry counts and timeouts

## Troubleshooting

### Common Issues

1. **Permission Denied**:
   - Check file system permissions
   - Use appropriate directories (`/tmp`, user home)
   - Verify tool accessibility

2. **Command Not Found**:
   - Ensure required tools are installed
   - Check PATH environment
   - Use full command paths if needed

3. **Timeout Errors**:
   - Increase timeout values
   - Check system performance
   - Optimize operations for efficiency

4. **Safety Violations**:
   - Review safety policies
   - Use safer command alternatives
   - Adjust mission design

### Getting Help

- **Documentation**: Check the [User Guide](../docs/USER_GUIDE.md)
- **API Reference**: See [API Documentation](../docs/api/README.md)
- **Issues**: Report problems on [GitHub Issues](https://github.com/your-org/rustchain/issues)
- **Community**: Join the [RustChain Forum](https://forum.rustchain.dev)

## Contributing Examples

We welcome contributions of new examples! Please:

1. **Follow the Format**: Use the standard mission structure
2. **Test Thoroughly**: Ensure examples work reliably
3. **Document Well**: Include clear descriptions and usage instructions
4. **Consider Safety**: Ensure examples are safe and educational
5. **Submit PR**: Submit via GitHub Pull Request

### Example Contribution Checklist

- [ ] Mission file is valid JSON
- [ ] All required tools are commonly available
- [ ] Mission executes successfully on clean system
- [ ] Documentation is complete and clear
- [ ] Safety validation passes
- [ ] Appropriate difficulty level assigned
- [ ] Estimated duration is accurate

Thank you for helping make RustChain more accessible and educational!
