use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, String>
}

pub trait Crud {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str) -> bool;
    fn update(&mut self, key: &str, new_value: &str) -> bool;
    fn delete(&mut self, key: &str) -> bool;
}

impl Crud for Database {
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }
    
    fn get(&self, key: &str) -> Option<String> {
        println!("{:#?}", self.data);
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) -> bool {
        if self.data.contains_key(key) {
            false
        } else {
            self.data.insert(key.to_string(), value.to_string());
            true
        }
    }

    fn update(&mut self, key: &str, new_value: &str) -> bool {
        if self.data.contains_key(key) {
            self.data.insert(key.to_string(), new_value.to_string());
            true
        }
        else {
            false
        }
        
    }

    fn delete(&mut self, key: &str) -> bool {
        if self.data.contains_key(key) {
            self.data.remove(key);
            true
        }
        else {
            false
        }
    }
}
