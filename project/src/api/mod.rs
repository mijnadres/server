pub mod routes;

#[derive(Serialize, Deserialize, Debug, PartialEq, Queryable)]
pub struct ContactRecord {
    id: i32,
    name: String,
    phone: String,
}

impl ContactRecord {
    pub fn new(name: &str, phone: &str) -> ContactRecord {
        ContactRecord {
            id: 0,
            name: name.to_string(),
            phone: phone.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn should_be_able_to_serialize_to_json() {
        let original = ContactRecord::new("test", "Hello, World!");

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }
}
