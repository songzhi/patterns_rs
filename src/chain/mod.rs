//! # 职责链模式
//!
//! 职责链模式用于分离不同职责，并且动态组合相关职责。
//!
//! Rust实现职责链模式时候，因为没有继承的支持，使用链对象包涵职责的方式，即：
//!
//! * 链对象包含当前职责对象以及下一个职责链。
//! * 职责对象提供接口表示是否能处理对应请求。
//! * 职责对象提供处理函数处理相关职责。
//!
//! 同时可在职责链类中实现职责接口相关函数，使职责链对象可以当做一般职责对象是用。

pub trait Manager {
    fn have_right(&self, money: usize) -> bool;
    fn handle_fee_request(&self, name: &str, money: usize) -> bool;
}


pub struct RequestChain {
    manager: Box<dyn Manager>,
    successor: Option<Box<RequestChain>>,
}

impl Manager for RequestChain {
    fn have_right(&self, _: usize) -> bool {
        true
    }
    fn handle_fee_request(&self, name: &str, money: usize) -> bool {
        if self.manager.have_right(money) {
            self.manager.handle_fee_request(name, money)
        } else {
            self.successor.as_ref().map(|s| s.handle_fee_request(name, money)).unwrap_or(false)
        }
    }
}

impl RequestChain {
    pub fn with_manager(manager: Box<dyn Manager>) -> Self {
        Self {
            manager,
            successor: None,
        }
    }
}

#[derive(Default)]
pub struct ProjectManager {}

impl Manager for ProjectManager {
    fn have_right(&self, money: usize) -> bool {
        money < 500
    }

    fn handle_fee_request(&self, name: &str, money: usize) -> bool {
        if name.to_lowercase() == "bob" {
            println!("Project Manager permit {}'s {} fee request", name, money);
            true
        } else {
            println!("Project Manager don't permit {}'s {} fee request", name, money);
            false
        }
    }
}

#[derive(Default)]
pub struct DepManager {}

impl Manager for DepManager {
    fn have_right(&self, money: usize) -> bool {
        money < 5000
    }

    fn handle_fee_request(&self, name: &str, money: usize) -> bool {
        if name.to_lowercase() == "tom" {
            println!("Dep Manager permit {}'s {} fee request", name, money);
            true
        } else {
            println!("Dep Manager don't permit {}'s {} fee request", name, money);
            false
        }
    }
}

#[derive(Default)]
pub struct GeneralManager {}

impl Manager for GeneralManager {
    fn have_right(&self, _: usize) -> bool {
        true
    }

    fn handle_fee_request(&self, name: &str, money: usize) -> bool {
        if name.to_lowercase() == "ada" {
            println!("General Manager permit {}'s {} fee request", name, money);
            true
        } else {
            println!("General Manager don't permit {}'s {} fee request", name, money);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c1 = RequestChain::with_manager(Box::new(ProjectManager::default()));
        let mut c2 = RequestChain::with_manager(Box::new(DepManager::default()));
        let c3 = RequestChain::with_manager(Box::new(GeneralManager::default()));
        c2.successor = Some(Box::new(c3));
        c1.successor = Some(Box::new(c2));

        let c: Box<dyn Manager> = Box::new(c1);
        c.handle_fee_request("bob", 400);
        c.handle_fee_request("tom", 1400);
        c.handle_fee_request("ada", 10000);
        c.handle_fee_request("floar", 400);
        // Output:
        // Project Manager permit bob's 400 fee request
        // Dep Manager permit tom's 1400 fee request
        // General Manager permit ada's 10000 fee request
        // Project Manager don't permit floar's 400 fee request
    }
}