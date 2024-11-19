use crate::token::{LoxLiteral, Token};
use std::collections::HashMap;

pub struct Environment {
    environments: Vec<HashMap<String, LoxLiteral>>,
}

impl Environment {
    pub fn new() -> Self {
        let global_env = HashMap::new();

        Environment {
            environments: vec![global_env],
        }
    }

    pub fn define(&mut self, name: String, value: LoxLiteral) {
        let target_env = self.environments.len() - 1;
        self.environments[target_env].insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Option<&LoxLiteral> {
        for env in self.environments.iter().rev() {
            match env.contains_key(&name.lexeme) {
                true => return env.get(&name.lexeme),
                false => {
                    continue;
                }
            }
        }
        None
    }

    pub fn assign(&mut self, name: String, value: LoxLiteral) -> Option<LoxLiteral> {
        for env in self.environments.iter_mut().rev() {
            match env.contains_key(&name) {
                true => {
                    return env.insert(name, value);
                }
                false => {
                    continue;
                }
            }
        }
        None
    }

    pub fn create_environment(&mut self) {
        self.environments.push(HashMap::new());
    }

    pub fn delete_environment(&mut self) {
        self.environments.pop();
    }
}
