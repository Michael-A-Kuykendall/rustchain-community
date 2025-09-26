# RustChain

A Rust-based workflow execution and transpilation framework.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-passing-green.svg)](https://github.com/Michael-A-Kuykendall/rustchain-community/actions)

## Overview

RustChain is a workflow execution framework written in Rust that can execute multi-step workflows and convert between different workflow formats.

## Features

- Execute workflows defined in YAML format
- Transpile between workflow platforms (GitHub Actions, Airflow, Jenkins, etc.)
- Built-in safety validation and error handling
- Memory-safe execution with Rust's ownership system
- Comprehensive test suite

## Installation

```bash
cargo install rustchain-community
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
rustchain-community = "1.0"
```

## Quick Start

Create a workflow file:

```yaml
name: "hello world"
version: "1.0"
steps:
  - id: "hello"
    name: "Say hello"
    type: "command"
    parameters:
      command: "echo"
      args: ["Hello, World!"]
```

Run the workflow:

```bash
rustchain run workflow.yaml
```

## Documentation

- [Installation Guide](docs/installation.md)
- [Usage Guide](docs/usage-guide.md)
- [API Reference](docs/api-reference.md)
- [Examples](examples/)

## Supported Formats

RustChain can read and write workflows in these formats:

- GitHub Actions
- Apache Airflow
- Jenkins
- Kubernetes
- Terraform
- Bash scripts

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under either the MIT or Apache-2.0 license, at your option.