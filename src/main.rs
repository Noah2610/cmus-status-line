extern crate dirs;
extern crate htmlescape;
extern crate regex;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate toml;

mod action;
mod args;
mod cmus_status;
mod config;
mod error;
mod meta;

fn main() {
    use std::process;

    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn run() -> error::MyResult<()> {
    use action::prelude::*;

    match action()? {
        Action::Status => cmus_status::print_cmus_status(),
        Action::Help => {
            print_help();
            Ok(())
        }
        Action::Version => {
            print_version();
            Ok(())
        }
        Action::DumpConfig => {
            dump_config();
            Ok(())
        }
    }
}

fn print_help() {
    let opt_help = {
        let opt = args::CliOption::Help;
        format!("-{}, --{}", opt.name_single(), opt.name_double())
    };
    let opt_vers = {
        let opt = args::CliOption::Version;
        format!("-{}, --{}", opt.name_single(), opt.name_double())
    };
    let cmd_status = args::CliCommand::Status.name();
    let cmd_help = args::CliCommand::Help.name();
    let cmd_dump_config = args::CliCommand::DumpConfig.name();

    println!(
        r#"{description}

USAGE:
    {name} [OPTIONS] [COMMAND]

OPTIONS:
    {opt_help:<opt_width$} Print this help message and exit.
    {opt_vers:<opt_width$} Print version information and exit.

COMMANDS:
    {cmd_status}
        Print the current cmus playback status
        with the format configured in the config.toml file.
        This is the default command, so you may omit this argument.
    {cmd_dump_config}
        Print the default config as TOML to stdout.
        To write the default config to the proper config file, run something like:
            mkdir -p ~/.config/{name}
            {name} {cmd_dump_config} > ~/.config/{name}/config.toml
    {cmd_help}
        Print this help message and exit."#,
        description = meta::DESCRIPTION,
        name = meta::NAME,
        opt_width = 16,
        opt_help = opt_help,
        opt_vers = opt_vers,
        cmd_status = cmd_status,
        cmd_help = cmd_help,
        cmd_dump_config = cmd_dump_config,
    );
}

fn print_version() {
    println!("{} v{}", meta::NAME, meta::VERSION)
}

fn dump_config() {
    print!(
        r#"# DEFAULT CONFIG FOR {name}
# To write this config to the proper config file, run something like:
#     mkdir -p ~/.config/{name}
#     {name} {cmd_dump_config} > ~/.config/{name}/config.toml

{config}"#,
        name = meta::NAME,
        cmd_dump_config = args::CliCommand::DumpConfig.name(),
        config = config::DEFAULT_CONFIG
    );
}
