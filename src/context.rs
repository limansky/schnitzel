use std::collections::HashMap;

use minijinja::Value;

#[derive(Debug, Default)]
pub struct Context {
    data: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<Value>>(mut self, field: &str, value: T) -> Self {
        self.data.insert(field.to_string(), value.into());
        self
    }

    pub fn get(&self, field: &str) -> Option<&Value> {
        self.data.get(field)
    }
}

#[cfg(test)]
mod tests {
    use minijinja::Value;

    use crate::context::Context;

    #[test]
    fn context_base_test() {
        let ctx = Context::new().field("aaa", "a value").field("bbb", 42);
        assert_eq!(ctx.get("aaa"), Some(&Value::from("a value")));
        assert_eq!(ctx.get("bbb"), Some(&Value::from(42)));
        assert_eq!(ctx.get("ccc"), None);
    }
}
