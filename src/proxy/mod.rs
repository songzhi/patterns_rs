//!# 代理模式
//!
//!代理模式用于延迟处理操作或者在进行实际操作前后进行其它处理。
//!
//!## 代理模式的常见用法有
//!
//!* 虚代理
//!* COW代理
//!* 远程代理
//!* 保护代理
//!* Cache 代理
//!* 防火墙代理
//!* 同步代理
//!* 智能指引
//!

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