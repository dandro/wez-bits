# Wez Bits

CLI tool to load and run project specific integrations.

## Development

Use cargo for everything.
- build `cargo build`
- run `cargo run -- -h`

## Installation

Download repository and use cargo to install the tool. `cargo install --path .`

## Configuration

Create a `.wez-bits/config.toml` file in the project you are trying to integrate. The TOML file should follow the structure below:

```toml
# Common tasks
[build]
program = "npm"
args = ["run", "build"]

[format]
program = ""
args = []

[run]
program = ""
args = []

[test]
program = ""
args = []

# You can add any custom tasks you need
[custom_task]
program = "your-command"
args = ["arg1", "arg2"]
```

