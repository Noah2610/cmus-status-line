use super::*;

#[derive(Default)]
pub struct CmusStatusBuilder {
    data:   Option<CmusData>,
    format: Option<Format>,
}

impl CmusStatusBuilder {
    pub fn data(mut self, data: CmusData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn format<T>(mut self, into_format: T) -> Self
    where
        T: Into<Format>,
    {
        self.format = Some(into_format.into());
        self
    }

    pub fn build(self) -> MyResult<CmusStatus> {
        Ok(CmusStatus {
            data:   self.data.ok_or(Error::CmusStatusNoData)?,
            format: self.format.unwrap_or_else(Default::default),
        })
    }
}
