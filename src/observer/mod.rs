//!# 观察者模式
//!
//!观察者模式用于触发联动。
//!
//!一个对象的改变会触发其它观察者的相关动作，而此对象无需关心连动对象的具体实现。
//!

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
pub struct Subject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    context: String,
}

impl Subject {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.borrow_mut().update(self);
        }
    }
    pub fn update_context(&mut self, context: String) {
        self.context = context;
        self.notify();
    }
}

pub trait Observer {
    fn update(&mut self, subject: &Subject);
}

#[derive(Default)]
struct Reader {
    name: String
}

impl Observer for Reader {
    fn update(&mut self, subject: &Subject) {
        self.name = subject.context.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut subject = Subject::new();
        let reader1 = Rc::new(RefCell::new(Reader::default()));
        let reader2 = Rc::new(RefCell::new(Reader::default()));
        let reader3 = Rc::new(RefCell::new(Reader::default()));
        subject.attach(reader1.clone());
        subject.attach(reader2.clone());
        subject.attach(reader3.clone());

        subject.update_context("observer mode".to_string());
        assert_eq!(reader1.borrow().name.as_str(), "observer mode");
        assert_eq!(reader2.borrow().name.as_str(), "observer mode");
        assert_eq!(reader3.borrow().name.as_str(), "observer mode");
    }
}