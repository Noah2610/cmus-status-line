use super::*;

#[derive(Default)]
pub struct CmusStatusBuilder {
    data:     Option<CmusData>,
    format:   Option<Format>,
    settings: Option<Settings>,
}

impl CmusStatusBuilder {
    pub fn data(mut self, data: CmusData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn format<T>(mut self, into_format: T) -> MyResult<Self>
    where
        T: std::convert::TryInto<Format>,
        Error: std::convert::From<<T as std::convert::TryInto<Format>>::Error>,
    {
        self.format = Some(into_format.try_into()?);
        Ok(self)
    }

    pub fn settings(mut self, settings: Settings) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn build(self) -> MyResult<CmusStatus> {
        Ok(CmusStatus {
            data:     self.data.ok_or(Error::CmusStatusNoData)?,
            format:   self.format.unwrap_or_else(Default::default),
            settings: self.settings.unwrap_or_else(Default::default),
        })
    }
}
