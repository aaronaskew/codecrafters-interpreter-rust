use std::collections::HashMap;

use crate::{RuntimeError, Value, token::Token};

#[derive(Debug, Default)]
pub(crate) struct Environment {
    values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub(crate) fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value);
    }

    pub(crate) fn get(&self, name: &Token) -> Result<&Option<Value>, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value)
        } else {
            Err(RuntimeError::new(
                &format!("Undefined variable '{}'.", name.lexeme),
                &name,
            ))
        }
    }
}
