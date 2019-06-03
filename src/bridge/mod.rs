//! # 桥接模式
//!
//! 桥接模式分离抽象部分和实现部分。使得两部分独立扩展。
//!
//! 桥接模式类似于策略模式，区别在于策略模式封装一系列算法使得算法可以互相替换。
//!
//! 策略模式使抽象部分和实现部分分离，可以独立变化。

pub trait AbstractMessage {
    fn send_message(&self, text: &str, to: &str) -> String;
}

pub trait MessageImplementer {
    fn send(&self, text: &str, to: &str) -> String;
}

pub struct MessageSMS {}

impl MessageSMS {
    pub fn via() -> Box<dyn MessageImplementer> {
        Box::new(Self {})
    }
}

impl MessageImplementer for MessageSMS {
    fn send(&self, text: &str, to: &str) -> String {
        format!("send {} to {} via SMS", text, to)
    }
}


pub struct MessageEmail {}

impl MessageEmail {
    pub fn via() -> Box<dyn MessageImplementer> {
        Box::new(Self {})
    }
}

impl MessageImplementer for MessageEmail {
    fn send(&self, text: &str, to: &str) -> String {
        format!("send {} to {} via Email", text, to)
    }
}

pub struct CommonMessage {
    method: Box<dyn MessageImplementer>
}

impl CommonMessage {
    pub fn new(method: Box<dyn MessageImplementer>) -> Self {
        Self {
            method
        }
    }
}

impl AbstractMessage for CommonMessage {
    fn send_message(&self, text: &str, to: &str) -> String {
        self.method.send(text, to)
    }
}

pub struct UrgencyMessage {
    method: Box<dyn MessageImplementer>
}

impl UrgencyMessage {
    pub fn new(method: Box<dyn MessageImplementer>) -> Self {
        Self {
            method
        }
    }
}

impl AbstractMessage for UrgencyMessage {
    fn send_message(&self, text: &str, to: &str) -> String {
        self.method.send(text, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_sms() {
        let m = CommonMessage::new(MessageSMS::via());
        assert_eq!(m.send_message("have a drink?", "bob"), "send have a drink? to bob via SMS")
    }

    #[test]
    fn test_urgency_email() {
        let m = UrgencyMessage::new(MessageEmail::via());
        assert_eq!(m.send_message("have a drink?", "bob"), "send have a drink? to bob via Email")
    }
}