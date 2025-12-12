use std::collections::HashMap;

pub struct Environment<V> {
    vars: HashMap<String, V>,
}

impl<V> Environment<V> {

    pub fn new() -> Environment<V> {
        Environment{vars: HashMap::new()}
    }

    pub fn declare(&mut self, name: &str, value: V) {
        // declare a new variable
        self.vars.insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&V> {
        self.vars.get(name)
    }

    pub fn assign(&mut self, name: &str, value: V) {
        // change value of an *already declared* variable (name=value)
        todo!()
    }
}