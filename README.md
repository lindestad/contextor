# contextor

Outline:
project-context-extractor/   # Root of your project
├── src/                     # Rust source code
│   ├── main.rs              # Entry point
│   ├── app.rs               # GUI logic (UI handling)
│   ├── scanner.rs           # Handles file scanning, .gitignore parsing
│   ├── formatter.rs         # Formats extracted content into structured output
│   ├── clipboard.rs         # Handles copying output to clipboard
│   ├── utils.rs             # Miscellaneous helper functions
│   ├── config.rs            # Handles configuration settings (future growth)
│   ├── tests/               # Integration and unit tests
│   │   ├── scanner_tests.rs # Unit tests for file scanning
│   │   ├── formatter_tests.rs # Tests for output formatting
│   │   ├── gui_tests.rs     # (If testing UI behavior with mock inputs)
│   └── prelude.rs           # Common imports for easy re-use
│
├── assets/                  # (Optional) Icons, logos, or UI assets
│   ├── icon.png
│   ├── logo.svg
│
├── target/                  # Build output (ignored in Git)
│
├── Cargo.toml               # Rust dependencies and metadata
├── Cargo.lock               # Lock file for dependencies
├── .gitignore               # Ignore unnecessary files
├── README.md                # Project documentation
├── LICENSE                  # License file (MIT, Apache, etc.)
├── CONTRIBUTING.md          # Guidelines for contributors
├── CHANGELOG.md             # Tracks changes between versions
├── .github/                 # GitHub-specific settings
│   ├── ISSUE_TEMPLATE.md    # Template for bug reports & feature requests
│   ├── PULL_REQUEST_TEMPLATE.md # Template for PR submissions
│   ├── workflows/           # CI/CD automation (GitHub Actions)
│   │   ├── rust.yml         # CI pipeline for running tests & linting
│   │   ├── release.yml      # Automates releases (optional)
│
└── docs/                    # Documentation (for GitHub Pages or Rustdoc)
    ├── index.md
    ├── architecture.md      # High-level architecture overview
    ├── contributing.md      # How to contribute (linked from CONTRIBUTING.md)
    ├── usage.md             # How to use the program
