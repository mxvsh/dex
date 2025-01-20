# dex

A simple CLI tool to run commands in multiple directories simultaneously.

## Features

- Run the same command across multiple directories
- Parallel execution using threads
- Progress indicators for each directory
- Execution time tracking

## Installation

```bash
cargo install dex
```

## Usage

```bash
dex --command "npm install" --dirs "project1,project2,project3"
```

### Arguments

- `-c, --command`: The command to execute in each directory
- `-d, --dirs`: Comma-separated list of directories

### Example

```bash
# Run git status in multiple repositories
dex -c "git status" -d "repo1,repo2,repo3"

# Run npm install in multiple projects
dex -c "npm install" -d "frontend,backend,common"
```

## Development

1. Clone the repository
2. Build the project:

```bash
cargo build
```

3. Run tests:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License
