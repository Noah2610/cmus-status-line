mod data;
mod status;

use crate::error::prelude::*;
use std::convert::TryFrom;
use std::process::Command;

use data::CmusData;

// TODO
type CmusStatus = ();

pub fn get_cmus_status() -> MyResult<CmusStatus> {
    let output = get_cmus_remote_output()?;
    let cmus_data = CmusData::try_from(output)?;
    println!("{:#?}", &cmus_data);
    // CmusStatus::new(cmus_data)
    unimplemented!()
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
