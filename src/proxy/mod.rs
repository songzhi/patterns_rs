pub trait Subject {
    fn operation(&self) -> String;
}

struct RealSubject {}

impl Subject for RealSubject {
    fn operation(&self) -> String {
        "real".to_string()
    }
}

pub struct Proxy {
    real: RealSubject
}

impl Subject for Proxy {
    fn operation(&self) -> String {
        format!("pre:{}:after", self.real.operation())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let subject = Proxy {
            real: RealSubject {}
        };
        assert_eq!("pre:real:after", subject.operation().as_str())
    }
}