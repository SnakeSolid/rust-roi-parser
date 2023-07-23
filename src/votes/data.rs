use crate::database::VoteData;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use time::macros::format_description;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Reply {
    Error {
        success: bool,
        message: String,
    },
    List {
        success: bool,
        items: Vec<ReplyItem>,
    },
}

impl Reply {
    pub fn error(message: String) -> Self {
        Reply::Error {
            success: false,
            message,
        }
    }

    pub fn list(items: Vec<ReplyItem>) -> Self {
        Reply::List {
            success: true,
            items,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyItem {
    datetime: String,
    positive: u32,
    negative: u32,
}

impl TryFrom<(i64, u32, u32)> for ReplyItem {
    type Error = Box<dyn Error>;

    fn try_from(value: (i64, u32, u32)) -> Result<Self, Self::Error> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let datetime = OffsetDateTime::from_unix_timestamp(value.0)?;

        Ok(Self {
            datetime: datetime.format(&format)?,
            positive: value.1,
            negative: value.2,
        })
    }
}

impl TryFrom<VoteData> for ReplyItem {
    type Error = Box<dyn Error>;

    fn try_from(value: VoteData) -> Result<Self, Self::Error> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let datetime = OffsetDateTime::from_unix_timestamp(value.timestamp())?;

        Ok(Self {
            datetime: datetime.format(&format)?,
            positive: value.positive(),
            negative: value.negative(),
        })
    }
}

impl TryFrom<HourRow> for ReplyItem {
    type Error = Box<dyn Error>;

    fn try_from(value: HourRow) -> Result<Self, Self::Error> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let datetime = OffsetDateTime::from_unix_timestamp(value.timestamp())?;

        Ok(Self {
            datetime: datetime.format(&format)?,
            positive: value.positive(),
            negative: value.negative(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListParams {
    id: u32,
}

impl ListParams {
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyParams {
    id: u32,
    hours: u64,
}

impl HourlyParams {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn hours(&self) -> u64 {
        self.hours
    }
}

#[derive(Debug, Clone)]
pub struct HourRow {
    timestamp: i64,
    positive: u32,
    negative: u32,
}

impl HourRow {
    pub fn empty(timestamp: i64) -> Self {
        Self {
            timestamp,
            positive: 0,
            negative: 0,
        }
    }

    pub fn merge(&mut self, positive: u32, negative: u32) {
        self.positive = self.positive.max(positive);
        self.negative = self.negative.max(negative);
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn positive(&self) -> u32 {
        self.positive
    }

    pub fn negative(&self) -> u32 {
        self.negative
    }
}
