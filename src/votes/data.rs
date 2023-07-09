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

impl ReplyItem {
    pub fn new(datetime: String, positive: u32, negative: u32) -> Self {
        Self {
            datetime,
            positive,
            negative,
        }
    }

    pub fn datetime(&self) -> &str {
        &self.datetime
    }

    pub fn positive(&self) -> u32 {
        self.positive
    }

    pub fn negative(&self) -> u32 {
        self.negative
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ListParams {
    id: u32,
}

impl ListParams {
    pub fn id(&self) -> u32 {
        self.id
    }
}
