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


Usage: wzb <COMMAND>

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

Setup your project config file by running `wzb config create`. This creates a `.wez` directory with a `config.toml` file:

```toml
# WezBits Configuration

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

[check]
program = ""
args = []

# Interactive registers
[q]
program = ""
args = []

[w]
program = ""
args = []

[e]
program = ""
args = []

[y]
program = ""
args = []

# Non-interactive registers
[Q]
program = ""
args = []

[W]
program = ""
args = []

[E]
program = ""
args = []

[Y]
program = ""
args = []
```

View your current config with `wzb config view`.

## Running Tasks

```sh
wzb task-runner <NAME> [OPTIONS]

Arguments:
  <NAME>  Task name in config file

Options:
  -c, --close <CLOSE>        Configure when the task pane closes [default: on-success] [possible values: always, on-success, never]
  -d, --direction <DIR>      Direction to open the panel [default: right] [possible values: right, down]
  -p, --param <KEY=VALUE>    Values the command can use when executed (repeatable)
```

### Params

Tasks can declare dynamic values using `%{key}` placeholders in their `args`. Pass values at runtime with `--param key=value` (repeatable for multiple params).

Config:
```toml
[deploy]
program = "kubectl"
args = ["apply", "-f", "%{manifest}", "--namespace", "%{env}"]
```

Command:
```sh
wzb task-runner deploy --param manifest=app.yaml --param env=production
```

### Examples

```sh
# Run the build task
wzb task-runner build

# Run tests, keeping the pane open always
wzb task-runner test --close never

# Run a task in a pane below
wzb task-runner run --direction down

# Run a task with params
wzb task-runner deploy --param env=production --param manifest=app.yaml
```
