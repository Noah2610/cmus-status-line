mod data;
pub mod output;

use crate::error::prelude::*;
use std::convert::TryFrom;
use std::process::Command;

use data::CmusData;
use output::StatusOutput;

pub fn print_cmus_status() -> MyResult<()> {
    let cmus_status = get_cmus_status()?;
    println!("{}", cmus_status);
    Ok(())
}

pub fn get_cmus_status() -> MyResult<StatusOutput> {
    let output = get_cmus_remote_output()?;
    let cmus_data = CmusData::try_from(output)?;
    let config = crate::config::get_config()?;
    StatusOutput::builder()
        .data(cmus_data)
        .format(config.format)
        .config(config.output)
        .build()
}

fn get_cmus_remote_output() -> MyResult<String> {
    match Command::new("cmus-remote").arg("-Q").output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                Err(Error::CmusError {
                    status: output.status,
                    stderr: String::from_utf8(output.stderr).unwrap(),
                })
            }
        }
        Err(_) => Err(Error::CmusNotInstalled),
    }
}
