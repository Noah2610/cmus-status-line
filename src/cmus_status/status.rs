use super::data::prelude::*;
use crate::error::prelude::*;
use std::fmt;

pub struct CmusStatus {
    data: CmusData,
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

#[derive(Default)]
pub struct CmusStatusBuilder {
    data: Option<CmusData>,
}

impl CmusStatusBuilder {
    pub fn data(mut self, data: CmusData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> MyResult<CmusStatus> {
        Ok(CmusStatus {
            data: self.data.ok_or(Error::CmusStatusNoData)?,
        })
    }
}
