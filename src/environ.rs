use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment<V: Clone> {
    parent: Option<Rc<Environment<V>>>,
    vars: RefCell<HashMap<String, V>>,
}

impl<V: Clone> Environment<V> {

    pub fn new(parent: Option<Rc<Environment<V>>>) -> Rc<Environment<V>> {
        Rc::new(Environment{ parent, vars: HashMap::new().into()})
    }

    pub fn declare(&self, name: &str, value: V) {
        // declare a new variable
        self.vars.borrow_mut().insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<V> {
        Some(self.vars.borrow().get(name)?.clone())
    }

    pub fn assign(&self, name: &str, value: V) {
        // change value of an *already declared* variable (name=value)
        // needs error checking
        self.vars.borrow_mut().insert(name.into(), value);
    }
}