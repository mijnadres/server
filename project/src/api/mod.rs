pub struct Event {
    origin: String,
    message: String,
}

impl Event {
    pub fn new(origin: &str, message: &str) -> Event {
        Event {
            origin: origin.to_string(),
            message: message.to_string(),
        }
    }
}
