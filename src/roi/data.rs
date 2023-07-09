#[derive(Debug)]
pub struct InitiativeData {
    id: u32,
    name: String,
    is_archived: bool,
    positive: u32,
    negative: u32,
}

impl InitiativeData {
    pub fn new(id: u32, name: &str, is_archived: bool, positive: u32, negative: u32) -> Self {
        Self {
            id,
            name: name.into(),
            is_archived,
            positive,
            negative,
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

    pub fn positive(&self) -> u32 {
        self.positive
    }

    pub fn negative(&self) -> u32 {
        self.negative
    }
}
