use crate::database::InitiativeData;
use serde::Deserialize;
use serde::Serialize;

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
    Success {
        success: bool,
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

    pub fn success() -> Self {
        Reply::Success { success: true }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyItem {
    id: u32,
    name: String,
    is_archived: bool,
    is_active: bool,
}

impl ReplyItem {
    pub fn new(id: u32, name: String, is_archived: bool, is_active: bool) -> Self {
        Self {
            id,
            name,
            is_archived,
            is_active,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl From<InitiativeData> for ReplyItem {
    fn from(value: InitiativeData) -> Self {
        Self {
            id: value.initiative_id(),
            name: value.name().into(),
            is_archived: value.is_archived(),
            is_active: value.is_active(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableParams {
    id: u32,
}

impl EnableParams {
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisableParams {
    id: u32,
}

impl DisableParams {
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddParams {
    id: u32,
}

impl AddParams {
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveParams {
    id: u32,
}

impl RemoveParams {
    pub fn id(&self) -> u32 {
        self.id
    }
}
