use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{RuntimeError, Value, token::Token};

#[derive(Debug, Default, Clone)]
pub(crate) struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub(crate) fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing: enclosing.map(|e| Rc::clone(&e)),
            values: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value);
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Option<Value>, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            if let Some(enclosing) = &self.enclosing.as_ref().map(|e| e.borrow()) {
                let value = enclosing.get(name)?.clone();
                return Ok(value.clone());
            }

            Err(RuntimeError::new(
                &format!("Undefined variable '{}'.", name.lexeme),
                name,
            ))
        }
    }

    pub(crate) fn assign(&mut self, name: &Token, value: &Value) -> Result<Value, RuntimeError> {
        if let Some(val) = self.values.get_mut(&name.lexeme) {
            *val = Some(value.clone());
            Ok(value.clone())
        } else {
            if let Some(mut enclosing) = self.enclosing.as_ref().map(|e| e.borrow_mut()) {
                return enclosing.assign(name, value);
            }

            Err(RuntimeError::new(
                &format!("Undefined variable '{}'.", name.lexeme),
                name,
            ))
        }
    }
}
