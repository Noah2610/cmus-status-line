use super::*;

#[derive(Default)]
pub struct StatusOutputBuilder {
    data:   Option<CmusData>,
    format: Option<Format>,
}

impl StatusOutputBuilder {
    pub fn data(mut self, data: CmusData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> MyResult<StatusOutput> {
        Ok(StatusOutput {
            data:   self.data.ok_or(Error::CmusStatusNoData)?,
            format: self.format.unwrap_or_else(Default::default),
        })
    }
}
