# Rust ETL Engine

## Overview

This project provides a modern **ETL (Extract, Transform, Load) engine** written in **Rust**.  
It is designed to process large volumes of data with **high performance**, **memory safety**, and a **modular architecture**.  
The approach emphasizes **simplicity**, **extensibility**, and **community collaboration**.

## Project Goals

- Provide a lightweight and fast ETL tool
- Enable modular and extensible pipelines
- Ensure data reliability with Rust's memory safety
- Support both **batch** and **streaming** processing
- Serve as a learning and practical resource for Rust data engineering

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.80 or higher recommended)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Steps

```bash
# Clone the repository
git clone https://github.com/<organization>/<repo>.git
cd <repo>

# Build in debug mode
cargo build

# Run the project
cargo run -- run --config examples/pipeline.yml

# Build in release mode
cargo build --release
```

## Available Commands

- `cargo run -- run --config <file>` : runs a pipeline from a config file
- `cargo test` : runs unit and integration tests
- `cargo fmt` : formats code with rustfmt
- `cargo clippy` : lints code for best practices

## Project Structure

```bash
.
├── src/                # Main source code
│   ├── config.rs       # Configuration parsing (YAML/JSON)
│   ├── pipeline.rs     # Pipeline orchestration
│   ├── extractors/     # Data sources (CSV, PostgreSQL, HTTP, etc.)
│   ├── transformers/   # Transformations (filters, mapping, aggregation)
│   ├── loaders/        # Data sinks (CSV, PostgreSQL, Parquet)
│   └── utils.rs        # Utilities, logging, error handling
├── examples/           # Example pipeline configs
├── tests/              # Unit and integration tests
├── Cargo.toml
└── README.md
```

## Development Conventions

### Branches

- `main` : stable version ready for release
- `develop` : main development branch
- `feature/<name>` : for each new feature
- `fix/<name>` : for bug fixes
- `release/<version>` : preparing a stable release

### Commits

Follow this format:

```bash
<type>(<scope>): <message>
```

Examples:

```bash
feat(transformers): add groupby aggregation
fix(extractor-csv): handle missing header
docs(readme): update installation instructions
```

Common types:
- `feat` : new feature
- `fix` : bug fix
- `docs` : documentation
- `test` : adding or modifying tests
- `refactor` : code improvement without adding functionality
- `chore` : maintenance

### Push Rules

- Always create a dedicated branch before developing
- Create pull requests to `develop`
- Pull requests must be reviewed and approved by at least one team member
- Squash & merge recommended to keep history clean

## Contributing

### Process

1. Fork the project
2. Create a branch `feature/my-feature`
3. Implement your feature with tests
4. Run code style checks: `cargo fmt && cargo clippy`
5. Submit a pull request to `develop`

### Best Practices

- Use idiomatic Rust and follow Clippy recommendations
- Add unit tests for each new feature
- Keep modules small and focused
- Document code with clear comments

## Testing

Tests are written using the Rust built-in test framework.

Run tests:

```bash
cargo test
```

## License

This project is licensed under the MIT License.  
You can use, modify, and distribute it freely, provided the license notice is retained.

## Roadmap

- ☐ CSV extractor and loader
- ☐ PostgreSQL connector
- ☐ Basic transformations (filter, map)
- ☐ GroupBy and aggregation
- ☐ Parquet/Arrow support
- ☐ Streaming mode (Kafka, WebSockets)
- ☐ Web dashboard for monitoring