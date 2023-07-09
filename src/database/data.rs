#[derive(Debug)]
pub struct InitiativeData {
    initiative_id: u32,
    name: String,
    is_archived: bool,
    is_active: bool,
}

impl InitiativeData {
    pub fn new(initiative_id: u32, name: String, is_archived: bool, is_active: bool) -> Self {
        Self {
            initiative_id,
            name,
            is_archived,
            is_active,
        }
    }

    pub fn initiative_id(&self) -> u32 {
        self.initiative_id
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

#[derive(Debug)]
pub struct VoteData {
    initiative_id: u32,
    timestamp: i64,
    positive: u32,
    negative: u32,
}

impl VoteData {
    pub fn new(initiative_id: u32, timestamp: i64, positive: u32, negative: u32) -> Self {
        Self {
            initiative_id,
            timestamp,
            positive,
            negative,
        }
    }

    pub fn initiative_id(&self) -> u32 {
        self.initiative_id
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
