# Contributing to AIRun

Thank you for your interest in contributing to AIRun! This document provides guidelines and instructions for contributing.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear, descriptive title
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Your operating system and Rust version

### Suggesting Features

Feature suggestions are welcome! Please open an issue with:
- A clear, descriptive title
- A detailed description of the proposed feature
- Any relevant examples or use cases

### Adding New AI Tools

To add support for a new AI tool, edit `src/main.rs` and add a new `AiTool` entry to the `TOOLS` array:

```rust
AiTool {
    name: "Tool Name",
    command: "tool-command",
    description: "Brief description of the tool",
},
```

The tool will be automatically detected if the command is available in the user's PATH.

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Test your changes (`cargo test` if tests exist, or manual testing)
5. Commit your changes (`git commit -m "Add your feature"`)
6. Push to your branch (`git push origin feature/your-feature`)
7. Open a pull request

### Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Ensure code compiles without warnings (`cargo clippy`)
- Add comments where appropriate
- Keep the codebase simple and readable

## Development Setup

```bash
git clone https://github.com/ThomasNowProductions/AIRun
cd AIRun
cargo build
```

## Questions?

Feel free to open an issue for any questions or clarifications.

Thank you for contributing!
