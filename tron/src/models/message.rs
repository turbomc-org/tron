pub struct Message {
    pub title: String,
    pub body: String,
    pub actions: Vec<String>,
    pub message_type: MessageType,
}

pub struct Action {
    pub label: String,
    pub command: String,
}

impl Message {
    pub fn render(&self) -> String {
        match self.message_type {
            MessageType::Info => {
                format!("hello world")
            }
            MessageType::Warning => {
                format!("hello world")
            }
            MessageType::Error => {
                format!("hello world")
            }
        }
    }
}

pub enum MessageType {
    Info,
    Warning,
    Error,
}
