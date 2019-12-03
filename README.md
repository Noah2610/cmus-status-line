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

The configuration has a `format` key, which is a string.  
<details open>
<summary>
The default format string looks like this:
</summary>

```
format = """
%{ If(
    Or(IsStatus(Playing), IsStatus(Paused)),
    Container([
        Container([
            If(IsStatus(Playing),
                Text(" ")),
            If(IsStatus(Paused),
                Text(" ")),
            If(IsStatus(Stopped),
                Text(" ")),
        ]),

        If(
            IsStatus(Playing),
            Container([
                Truncate(Title, 60),
                Text("  "),
                ProgressBar("<####---->"),
            ]),
        ),

        If(
            IsStatus(Paused),
            Container([
                Truncate(Title, 10),
                Text(" "),
                ProgressBar("<##->"),
            ]),
        ),
    ]),
)}
"""
```
</details>

Any plain text in the string is simply printed in the format,  
so a `format` string with this value:
```
format = "my cmus status!"
```
would simply print `my cmus status!`.  
To add dynamic content, you can use the `%{...}` syntax to inject information,  
for example:
```
format = "playing song: %{Title}"
```
would replace the `%{Title}` part with the currently playing song's title.  
We call the `Title` part a `FormatPart`.

### `FormatPart`
Any of the following format parts can be used  
in the `format` string inside `%{...}` blocks.  
They will be replaced with a string value.

- __`Text(String)`__  
  Returns the given string.
- __`Title`__  
  Returns the currently playing song's title.  
  Any underscores (`_`) will be replaced with spaces (` `).
- __`Status`__  
  Returns the current playback status (`CmusPlaybackStatus`),  
  which can be one of:
    - `Playing`
    - `Paused`
    - `Stopped`
- __`Truncate(FormatPart, usize)`__  
  Returns the wrapped `FormatPart`'s return string,  
  truncated to the given `usize` length.  
  Example: `Truncate(Title, 20)`  
  which will return the full title of the song,  
  if it has less than or exactly `20` characters.  
  If it has less, the title will be truncated to `20` characters,  
  with trailing `...` characters.
- __`HtmlEscape(FormatPart)`__  
  Uses the [`htmlescape::encode_minimal`][htmlescape_encode_minimal] function, to escape  
  any HTML syntax such as `<>&` from the wrapped `FormatPart`.  
  Example: `HtmlEscape(Title)`
- __`ProgressBar(String)`__  
  Returns a progress bar for the playback of the currently playing song.  
  The given string acts as a config for which characters to use.  
  The first and last characters of the string are used as the boundary characters of the bar.  
  The second and second to last characters are used as the _full_ and _empty_ characters.  
  The total length of the string is the length of the progress bar.  
  Example: `ProgressBar("<##-->")` will use `<>` as the bar boundary characters,  
  the `#` as the _full_ character, and the `-` as the _empty_ character.  
  The progress bar will have a length of `6` characters.
- __`Container(Vec<FormatPart>)`__  
  This wraps multiple `FormatPart`s into a single one.  
  Useful in combination with other `FormatPart`s.  
  Example:
  ```
  Truncate(Container([
      Text("progress: "),
      ProgressBar("<##-->"),
      Text(" title: "),
      Title,
  ]), 60)
  ```
  which will truncate the combined length of the bar,  
  the song title, and some static text to 60 characters or less.
- __`If(FormatExpression, FormatPart)`__  
  Returns the evaluated `FormatPart`, if the `FormatExpression` returns `true`.  
  See the section on `FormatExpression` for available expressions.  
  Example:
  ```
  Container([
      If(
          IsStatus(Playing),
          Title,
      ),
      If(
          IsStatus(Paused),
          Text("PAUSED"),
      ),
  ])
  ```

---

__TODO:__ Documentation, cleanup, crates.io upload!  
It's working fine right now, but the project's not finished.

[releases]:                  https://github.com/Noah2610/cmus-status-line/releases
[crates.io]:                 https://crates.io/crates/cmus-status-line
[htmlescape_encode_minimal]: https://docs.rs/htmlescape/0.3.1/htmlescape/fn.encode_minimal.html
