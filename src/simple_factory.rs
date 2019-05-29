pub trait API {
    fn operation(&self, s: &str) -> String;
}

pub struct ImlA {}

impl API for ImlA {
    fn operation(&self, s: &str) -> String {
        format!("ImplA s == {}", s)
    }
}

pub struct ImlB {}

impl API for ImlB {
    fn operation(&self, s: &str) -> String {
        format!("ImplB s == {}", s)
    }
}

pub struct Factory {}

impl Factory {
    pub fn create_api(condition: i32) -> Box<dyn API> {
        if condition == 1 {
            Box::new(ImlA {})
        } else {
            Box::new(ImlB {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_a() {
        let api = Factory::create_api(1);
        let s = api.operation("Hello");
        assert_eq!("ImplA s == Hello", s.as_str());
    }

    #[test]
    fn test_impl_b() {
        let api = Factory::create_api(2);
        let s = api.operation("Hello");
        assert_eq!("ImplB s == Hello", s.as_str());
    }
}