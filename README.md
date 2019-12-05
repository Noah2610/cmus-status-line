# cmus-status-line
<details>
<summary>
    Table of Contents
</summary>

- [Description](#description)
- [Installation](#installation)
  - [Binaries](#binaries)
  - [Install from crates.io](#install-from-cratesio)
- [Usage](#usage)
- [Configuration](#configuration)
  - [`FormatPart`](#formatpart)
  - [`FormatExpression`](#formatexpression)
- [License](#license)

---
</details>

## Description
Prints the current `cmus` playback status in a customizable format to stdout.  
Example output with default config:
```
$ cmus-status-line # When PLAYING
 Undertale - Megalovania  <###----->

$ cmus-status-line # When PAUSED
 Underta... <#-->
```

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

The default configuration is in the [`config.toml`][default_config] file.

### Simple configuration example
Here's a small and simple configuration example to get you started,  
if you don't want to / don't have the time to read the details:
```
format = """
%{Title} - %{ProgressBar("<####---->")}
"""
```

### The `format` key
The configuration has a `format` key, which is a string.  

Any plain text in the string is simply printed in the format,  
so a `format` string with this value:
```
format = "my cmus status!"
```
would simply print `my cmus status!`.  
Any new-line characters are ignored.  
To add dynamic content, you can use the `%{...}` syntax to inject information,  
for example:
```
format = "playing song: %{Title}"
```
would replace the `%{Title}` part with the currently playing song's title.  
We call the `Title` part a `FormatPart`.

### `FormatPart`
[`enum FormatPart`](https://github.com/Noah2610/cmus-status-line/blob/master/src/cmus_status/output/format/format_part.rs#L8)  
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

### `FormatExpression`
[`enum FormatExpression`](https://github.com/Noah2610/cmus-status-line/blob/master/src/cmus_status/output/format/format_expression.rs#L4)  
A `FormatExpression` can be used as the first argument to  
`If` `FormatPart`s. They will always evaluate to either `true` or `false`.

- __`True`__  
  Always returns `true`.

- __`False`__  
  Always returns `false`.

- __`And(FormatExpression, FormatExpression)`__  
  Returns `true` if both of the given `FormatExpression`s evaluate to `true`.

- __`Or(FormatExpression, FormatExpression)`__  
  Returns `true` if either of the given `FormatExpression`s evaluate to `true`.

- __`Not(FormatExpression)`__  
  Inverts the given expression.

- __`IsStatus(CmusPlaybackStatus)`__  
  Returns `true` if the given `CmusPlaybackStatus`  
  is the currently playing song's status.
  `CmusPlaybackStatus` can be one of:
    - `Playing`
    - `Paused`
    - `Stopped`

  Example:
  ```
  If(
      IsStatus(Playing),
      Container([
          Text("playing song: "),
          Title,
      ]),
  ),
  ```

---

## License
Distributed under the terms of the [MIT license][license].

[releases]:                  https://github.com/Noah2610/cmus-status-line/releases
[default_config]:            https://github.com/Noah2610/cmus-status-line/blob/master/config.toml
[crates.io]:                 https://crates.io/crates/cmus-status-line
[htmlescape_encode_minimal]: https://docs.rs/htmlescape/0.3.1/htmlescape/fn.encode_minimal.html
[license]:                   https://github.com/Noah2610/cmus-status-line/blob/master/LICENSE
