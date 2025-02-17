use std::fmt::{Debug, Display, Formatter};

pub struct Identifier {
    namespace: String,
    key: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            key: key.into(),
        }
    }

    pub fn default_namespace(key: impl Into<String>) -> Self {
        Self {
            namespace: "minecraft".to_string(),
            key: key.into(),
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::default_namespace("null")
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier{{ namespace: {}, key: {} }}", self.namespace, self.key)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        let split = value.split(":").collect::<Vec<_>>();

        match split.len() {
            1 => Identifier::default_namespace(split[0]),
            2 => Identifier::new(split[0], split[1]),
            _ => Identifier::default()
        }
    }
}