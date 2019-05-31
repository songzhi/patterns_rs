//! # 简单工厂模式
//!

pub trait API {
    fn say(&self, name: &str) -> String;
}

pub struct HiAPI {}

impl API for HiAPI {
    fn say(&self, name: &str) -> String {
        format!("Hi {}", name)
    }
}

pub struct HelloAPI {}

impl API for HelloAPI {
    fn say(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
}

pub fn create_api(condition: i32) -> Box<dyn API> {
    if condition == 1 {
        Box::new(HelloAPI {})
    } else {
        Box::new(HiAPI {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_a() {
        let api = create_api(1);
        let s = api.say("Jack");
        assert_eq!("Hello Jack", s.as_str());
    }

    #[test]
    fn test_impl_b() {
        let api = create_api(2);
        let s = api.say("Jack");
        assert_eq!("Hi Jack", s.as_str());
    }
}