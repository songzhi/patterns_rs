//! # 策略模式
//!
//! 定义一系列算法，让这些算法在运行时可以互换，使得分离算法，符合开闭原则。

pub struct PaymentContext {
    name: String,
    card_id: String,
    money: usize,
    payment_strategy: Box<dyn PaymentStrategy>,
}

pub trait PaymentStrategy {
    fn pay(&self, context: &PaymentContext) -> String;
}

impl PaymentContext {
    pub fn new(name: &str, card_id: &str, money: usize, payment_strategy: Box<dyn PaymentStrategy>) -> Self {
        Self {
            name: name.to_string(),
            card_id: card_id.to_string(),
            money,
            payment_strategy,
        }
    }
    pub fn pay(&self) -> String {
        self.payment_strategy.pay(self)
    }
}

pub struct Cash {}

impl PaymentStrategy for Cash {
    fn pay(&self, context: &PaymentContext) -> String {
        format!("Pay {} to {} by cash", context.money, context.name)
    }
}

pub struct Bank {}

impl PaymentStrategy for Bank {
    fn pay(&self, context: &PaymentContext) -> String {
        format!("Pay {} to {} by bank account {}", context.money, context.name, context.card_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cash = Box::new(Cash {});
        let bank = Box::new(Bank {});
        let mut context = PaymentContext::new("Ada", "D12", 123, cash);
        assert_eq!("Pay 123 to Ada by cash", context.pay().as_str());
        context.payment_strategy = bank;
        assert_eq!("Pay 123 to Ada by bank account D12", context.pay().as_str());
    }
}