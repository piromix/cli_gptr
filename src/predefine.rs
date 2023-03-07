use std::collections::HashMap;
use crate::file::read_config;
use serde::{Serialize, Deserialize};


pub struct Predefines {
    predefines: Option<HashMap<String, PredefineValue>>,

}
#[derive(Debug, Serialize, Deserialize)]
pub struct Predefine {
    predefine: HashMap<String, PredefineValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredefineValue {
    pub content: String,
    pub translate: bool,
}

impl PredefineValue {
    pub fn clone(self: &Self) -> Self {
        Self {
            content: self.content.clone(),
            translate: self.translate,
        }
    }

}


impl Predefines {
    pub fn new() -> Self {
        let define = read_config("gptr_predefine.json".to_string());
        match define {
            Some(define) => {
                let define: Predefine = serde_json::from_str(&define).unwrap();
                Self {
                    predefines: Some(define.predefine),
                }
            }
            None => {
                Self {
                    predefines: None,
                }
            }
        }
    }

    pub fn get_predefine(self: &Self, key: &String) -> Option<PredefineValue> {
        self.predefines.is_none().then(|| return None::<PredefineValue>);

        let pre = self.predefines.as_ref().unwrap();
        pre.contains_key(key).then(|| return pre.get(key).unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let predefines = Predefines::new();
        println!("{:?}", predefines.predefines);
    }
}