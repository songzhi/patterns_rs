//!# 组合模式
//!
//!组合模式统一对象和对象集，使得使用相同接口使用对象和对象集。
//!
//!组合模式常用于树状结构，用于统一叶子节点和树节点的访问，并且可以用于应用某一操作到所有子节点。

use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Rc<RefCell<dyn Component>>;
type WeakLink = Weak<RefCell<dyn Component>>;


pub trait Component {
    fn parent(&self) -> Option<WeakLink>;
    fn set_parent(&mut self, parent: WeakLink);
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn add_child(&mut self, child: Link) {}
    fn print(&self, pre: String);
}

pub struct Leaf {
    parent: WeakLink,
    name: String,
}

impl Leaf {
    pub fn new_link(name: String, parent: WeakLink) -> Link {
        Rc::new(RefCell::new(Self {
            name,
            parent,
        }))
    }
}

impl Component for Leaf {
    fn parent(&self) -> Option<WeakLink> {
        Some(self.parent.clone())
    }
    fn set_parent(&mut self, parent: WeakLink) {
        self.parent = parent;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn print(&self, pre: String) {
        println!("{}-{}", pre, self.name);
    }
}

#[derive(Default)]
pub struct Composite {
    parent: Option<WeakLink>,
    name: String,
    children: Vec<Link>,
}

impl Composite {
    pub fn new_link(name: String, parent: Option<WeakLink>) -> Link {
        Rc::new(RefCell::new(Self {
            name,
            parent,
            children: vec![],
        }))
    }
}

impl Component for Composite {
    fn parent(&self) -> Option<WeakLink> {
        self.parent.clone()
    }
    fn set_parent(&mut self, parent: WeakLink) {
        self.parent = Some(parent);
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn add_child(&mut self, child: Link) {
        self.children.push(child);
    }
    fn print(&self, mut pre: String) {
        println!("{}+{}", pre, self.name);
        pre.push(' ');
        for comp in self.children.iter() {
            comp.borrow().print(pre.clone());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Composite::new_link("root".to_string(), None);
        let c1 = Composite::new_link("c1".to_string(), Some(Rc::downgrade(&root)));
        root.borrow_mut().add_child(c1.clone());
        let c2 = Composite::new_link("c2".to_string(), Some(Rc::downgrade(&root)));
        root.borrow_mut().add_child(c2.clone());
        let c3 = Composite::new_link("c3".to_string(), Some(Rc::downgrade(&c1)));
        c1.borrow_mut().add_child(c3.clone());
        let l1 = Leaf::new_link("l1".to_string(), Rc::downgrade(&c1));
        c1.borrow_mut().add_child(l1.clone());
        let l2 = Leaf::new_link("l2".to_string(), Rc::downgrade(&c2));
        c2.borrow_mut().add_child(l2.clone());
        let l3 = Leaf::new_link("l3".to_string(), Rc::downgrade(&c2));
        c2.borrow_mut().add_child(l3.clone());

        root.borrow().print("".to_string());
        // Output:
        // +root
        //  +c1
        //   +c3
        //   -l1
        //  +c2
        //   -l2
        //   -l3
    }
}