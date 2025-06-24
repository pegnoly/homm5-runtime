pub struct CreateQuestPayload {
    pub mission_id: i32,
    pub name: String,
    pub script_name: String,
    pub directory: String
}

#[derive(Debug, Default)]
pub struct UpdateQuestPayload {
    pub id: i32,
    pub name: Option<String>,
    pub script_name: Option<String>,
    pub desc: Option<String>,
    pub directory: Option<String>,
    pub is_secondary: Option<bool>,
    pub is_active: Option<bool>
}

impl UpdateQuestPayload {
    pub fn new(id: i32) -> Self {
        UpdateQuestPayload {
            id,
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_script_name(mut self, script_name: String) -> Self {
        self.script_name = Some(script_name);
        self
    }

    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    pub fn with_directory(mut self, dir: String) -> Self {
        self.directory = Some(dir);
        self
    }

    pub fn with_secondary(mut self, secondary: bool) -> Self {
        self.is_secondary = Some(secondary);
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.is_active = Some(active);
        self
    }
}

pub struct GetProgressPayload {
    pub quest_id: i32,
    pub number: i32
}

pub struct SaveProgressPayload {
    pub id: i32,
    pub text: String,
    pub concatenate: bool
}