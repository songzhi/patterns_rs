pub trait Target {
    fn request(&self) -> String;
}

pub trait Adaptee {
    fn specific_request(&self) -> String;
}

pub fn new_adaptee() -> Box<dyn Adaptee> {
    Box::new(AdapteeImpl {})
}

struct AdapteeImpl {}

impl Adaptee for AdapteeImpl {
    fn specific_request(&self) -> String {
        String::from("adaptee method")
    }
}

struct Adapter {
    adaptee: Box<dyn Adaptee>
}


pub fn new_target(adaptee: Box<dyn Adaptee>) -> Box<dyn Target> {
    Box::new(Adapter { adaptee })
}


impl Target for Adapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let adaptee = new_adaptee();
        let target = new_target(adaptee);
        assert_eq!("adaptee method", target.request().as_str())
    }
}
