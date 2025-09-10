# Wez Bits

CLI tool to set up and run common and convenient commands that integrate and use the power of WezTerm.

```sh
                                         /$$       /$$   /$$
                                        | $$      |__/  | $$
 /$$  /$$  /$$  /$$$$$$  /$$$$$$$$      | $$$$$$$  /$$ /$$$$$$   /$$$$$$$
| $$ | $$ | $$ /$$__  $$|____ /$$/      | $$__  $$| $$|_  $$_/  /$$_____/
| $$ | $$ | $$| $$$$$$$$   /$$$$/       | $$  \ $$| $$  | $$   |  $$$$$$
| $$ | $$ | $$| $$_____/  /$$__/        | $$  | $$| $$  | $$ /$$\____  $$
|  $$$$$/$$$$/|  $$$$$$$ /$$$$$$$$      | $$$$$$$/| $$  |  $$$$//$$$$$$$/
 \_____/\___/  \_______/|________/      |_______/ |__/   \___/ |_______/


Harnessing WezTerm's Power


Usage: wez-bits <COMMAND>

Commands:
  task-runner  Run a project scoped task
  config       Interact with wez bits configuration
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Development

Use cargo for everything.
- build `cargo build`
- run `cargo run -- -h`

## Installation

Download repository and use cargo to install the tool. `cargo install --path .`

## Configuration

Setup your project config file by running `wez-bits confit create`. This should create a `.wez` directory and add a `config.toml` file that looks something like the below:

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

