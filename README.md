# monitorctl
A way to control monitor brightness through the command line. You can change the brightness of all monitors simultaneously.

## Features
- [X] Change brightness of all monitors 
- [X] Add multi-threading for perfomance boost
- [X] Use the `ddc-i2c` crate for perfomance improvement
- [X] Add/Subtract from the current value using `-a` and `-s` flags
- [ ] Change Contrast of all monitors
- [ ] Change Volume of all monitors 

## Installation
Install the dependencies first and then build and install the binary into PATH.

### Dependencies
1. DDC/CI feature need to be turned on in the monitors.
2. `i2c-dev` kernel module should be loaded during boot.
3. Add `~/.cargo/bin` to `$PATH` Environment variable.
Add `export PATH=$PATH:$HOME/.cargo/bin` to `~/.bash_profile` or `~/.zprofile` 
4. Rust Toolchain and git

###

### To preload `i2c-dev` in ArchLinux
1. Open `/etc/mkinitcpio` with the following command: `sudo vim /etc/mkinitcpio.conf`
2. Add `i2c-dev` to `MODULES` -> (`MODULES=(... i2c-dev)`)
3. Finally, run `sudo mkinitcpio -P` to generate linux kernel binaries
4. Reboot your computer.

>If the MODULES is empty in `/etc/mkinitcpio.conf` then, `MODULES=(i2c-dev)` should be enough.
> ... represents other modules that are present
    
### Build and Install
1. Clone this repo. `git clone https://github.com/5iddy/monitorctl.git`
2. Change directory to the repo. `cd monitorctl`
3. Run `make install`

## Usage
```
USAGE:
    monitorctl [OPTIONS] <Value>

ARGS:
    <Value>    

OPTIONS:
    -a, --add           Adds the value to the current value
    -h, --help          Print help information
    -s, --sub           Subs the value from the current value
    -V, --version       Print version information
```
### Examples:
1. `monitorctl 10` set brightness of all monitors to 10
2. `monitorctl -a 10` set brightness of all monitors to current value + 10
3. `monitorctl -s 10` set brightness of all monitors to current value - 10
