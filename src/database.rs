use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, String>
}

pub trait Crud {
    fn new() -> Self;
    fn get(&self, key: &String) -> Option<String>;
    fn set(&mut self, key: &String, value: String) -> bool;
    fn update(&mut self, key: &String, new_value: String) -> bool;
    fn delete(&mut self, key: &String) -> bool;
}

impl Crud for Database {
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }
    
    fn get(&self, key: &String) -> Option<String> {
        println!("{:#?}", self.data);
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: &String, value: String) -> bool {
        if self.data.contains_key(key) {
            false
        } else {
            self.data.insert(key.clone(), value);
            true
        }
    }

    fn update(&mut self, key: &String, new_value: String) -> bool {
        if self.data.contains_key(key) {
            println!("asdasd");
            self.data.insert(key.clone(), new_value);
            true
        }
        else {
            false
        }
        
    }

    fn delete(&mut self, key: &String) -> bool {
        if self.data.contains_key(key) {
            self.data.remove(key);
            true
        }
        else {
            false
        }
    }
}
