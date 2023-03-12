# monitorctl
A way to control monitor brightness through the command line. You can change the brightness of all monitors simultaneously.

## Features
- [X] Change brightness of all monitors 
- [X] Add multi-threading for perfomance boost
- [X] Use the `ddc-i2c` crate for perfomance improvement
- [X] Increase/Decrease from the current value using `-i` and `-d` flags
- [X] Change Contrast of all monitors
- [X] Change Volume of all monitors
- [ ] Add the ability to change the brightness of a laptop screen.

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
A command line tool to control monitor settings.
You can Increase/Decrease/Get/Set all connected monitors'
brightness, contrast, volume simultaneously.
Examples:
    monitorctl b 100   # set brightness to 100
    monitorctl b -i 10 # increase brightness by 10 points
    monitorctl b -d 10 # decrease brightness by 10 points
    monitorctl b -g    # get current brightness
    monitorctl c 100
    monitorctl c -i 10
    monitorctl c -d 10
    monitorctl c -g
    monitorctl v 100
    monitorctl v -i 10
    monitorctl v -d 10
    monitorctl v -g

Usage: monitorctl <COMMAND>

Commands:
  brightness  Change Brightness [aliases: b]
  contrast    Change Contrast [aliases: c]
  volume      Change Volume [aliases: v]
  debug       Debug Information
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
```
### brightness help
```
Change Brightness

Usage: monitorctl brightness [OPTIONS] [Value]

Arguments:
  [Value]  Value needed if '-i'/'--increase' or '-d'/'--decrease' flag is not used [default: 5]

Options:
  -g, --get-current-value  Get current value
  -i, --increase           Adds the value to the current value
  -d, --decrease           Subs the value from the current value
  -h, --help               Print help
  -V, --version            Print version
```
### Contrast help
```
Change Contrast

Usage: monitorctl contrast [OPTIONS] [Value]

Arguments:
  [Value]  Value needed if '-i'/'--increase' or '-d'/'--decrease' flag is not used [default: 5]

Options:
  -g, --get-current-value  Get current value
  -i, --increase           Adds the value to the current value
  -d, --decrease           Subs the value from the current value
  -h, --help               Print help
  -V, --version            Print version
```
### Volume help
```
Change Volume

Usage: monitorctl volume [OPTIONS] [Value]

Arguments:
  [Value]  Value needed if '-i'/'--increase' or '-d'/'--decrease' flag is not used [default: 5]

Options:
  -g, --get-current-value  Get current value
  -i, --increase           Adds the value to the current value
  -d, --decrease           Subs the value from the current value
  -h, --help               Print help
  -V, --version            Print version
```

Note: Volume sub command doesnt change pulseaudio or local volume, it changes the volume of the inbuilt monitor speaker which would have been otherwise set in panel menu.