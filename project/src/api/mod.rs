pub mod routes;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn should_be_able_to_serialize_to_json() {
        let original = Event::new("test", "Hello, World!");

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }
}
