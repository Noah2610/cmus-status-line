# cmus-status-line
## Description
Prints the current `cmus` playback status in a customizable format to stdout.

<details>
<summary>
    Example output with default config
</summary>

```
$ cmus-status-line # When PLAYING
 Undertale - Megalovania  <###----->

$ cmus-status-line # When PAUSED
 Underta... <#-->
```
</details>

## Installation
### Binaries
__TODO: Build and upload binaries!__  
Binaries for __Linux__ and __Windows__ are available
from the [GitHub releases][releases] page.

### Install from [crates.io]
__TODO: Upload to crates.io!__  
```
cargo install cmus-status-line
```

## Usage
Simply run the command without any arguments  
to get the formatted cmus playback status:
```
$ cmus-status-line
 Undertale - Megalovania  <###----->
```

For more details, see `cmus-status-line --help`:
```
Prints cmus playback information in a configurable format to stdout

USAGE:
    cmus-status-line [OPTIONS] [COMMAND]

OPTIONS:
    -h, --help       Print this help message and exit.
    -v, --version    Print version information and exit.

COMMANDS:
    status
        Print the current cmus playback status
        with the format configured in the config.toml file.
        This is the default command, so you may omit this argument.
    dump-config
        Print the default config as TOML to stdout.
        To write the default config to the proper config file, run something like:
            mkdir -p ~/.config/cmus-status-line
            cmus-status-line dump-config > ~/.config/cmus-status-line/config.toml
    help
        Print this help message and exit.
```

## Configuration
The goal for this project, was to make the status line's format highly configurable.  
You can configure the format as a string in the `config.toml` file.  
To get started, run the following to dump the default config to the proper config directory:  
(This assumes you are on Linux, for Windows or MacOS find your appropriate config directory here:  
https://docs.rs/dirs/2.0.2/dirs/fn.config_dir.html)
```
mkdir -p ~/.config/cmus-status-line
cmus-status-line dump-config > ~/.config/cmus-status-line/config.toml
```

---

__TODO:__ Documentation, cleanup, crates.io upload!  
It's working fine right now, but the project's not finished.

[releases]:  https://github.com/Noah2610/cmus-status-line/releases
[crates.io]: https://crates.io/crates/cmus-status-line
