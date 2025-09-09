# Wez Bits

CLI tool to load and run project specific integrations.

## Development

Use cargo for everything.
- build `cargo build`
- run `cargo run -- -h`

## Installation

Download repository and use cargo to install the tool. `cargo install --path .`

## Configuration

Create a `.wez-bits/config.json` file in the project you are trying to integrate. The JSON file should follow the structure below:

``` jsonc
{
  "build": {
    "program": "npm",
    "args": ["run", "build"]
  },
  "format": {}, // same structure
  "run": {}, // same structure
  "test": {} // same structure
}

```

