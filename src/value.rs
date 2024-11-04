#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int32(u32),
    Int64(u64),
    String(String),
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Int32(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Int64(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}
