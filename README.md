# cli-debug

cli-debug is a Rust tool designed to simplify debugging tasks by allowing users to open an SSH session to a remote host, execute a command, retrieve its output, and display it in the console. 
    While still a work in progress, cli-debug currently supports manually supplied as parameter one-time ran commands and terminates after execution. Future plans include expanding functionality to support interactive commands and potentially improving the user interface for better data visualization.

## Installation

To install cli-debug, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/ddelsizov/cli-debug.git
cd cli-debug
cargo build --release 
```
## Usage

```bash
cli-debug --target <TARGET> --user <USER> --pass <PASS> --cmd <CMD>
```
## Options
``` bash
-t, --target <TARGET>: Specifies the target host.
-u, --user <USER>: Specifies the username for SSH authentication.
-p, --pass <PASS>: Specifies the password for SSH authentication.
-c, --cmd <CMD>: Specifies the command to execute remotely.
-h, --help: Prints help information.
-V, --version: Prints version information.
```
## Note
At its current state, cli-debug doesn't offer significant advantages over running SSH commands directly, such as:
```bash
ssh user@host "whoami"
```