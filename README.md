# monitorctl
A way to control monitor brightness through the command line. You can change the brightness of all monitors through ddcutil. This program is a wrapper around the ddcutil command.

## Installation
Install the dependencies first and then build and install the binary into PATH.
### Dependencies
1. ddcutil -> `sudo pacman -S ddcutil`
2. DDC/CI feature need to be turned on in the monitors.

### Build and Install
1. `git clone https://github.com/5iddy/monitorctl.git`
2. `cd monitorctl`
3. `cargo build --release`
4. Copy the binary to a folder that is included in the path
    `cp target/release/monitorctl ~/.bin/`
                    or
    `cp target/release/monitorctl ~/.local/bin`
                    or
    `cargo install --path .`

## Usage
`monitorctl <brightness-value>`
