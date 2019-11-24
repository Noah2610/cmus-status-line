mod builder;
mod format;

use super::data::prelude::*;
use crate::error::prelude::*;
use std::fmt;

use builder::CmusStatusBuilder;
use format::Format;

pub struct CmusStatus {
    data:   CmusData,
    format: Format,
}

impl CmusStatus {
    pub fn builder() -> CmusStatusBuilder {
        CmusStatusBuilder::default()
    }
}

impl fmt::Display for CmusStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO")
    }
}
