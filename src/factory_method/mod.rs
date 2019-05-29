//! ## 工厂方法模式
//! ### 功能
//! 工厂方法的主要功能是让父类在不知道具体实现的情况下，完成自身的功能调用，而具体的实现延迟到子类来实现。
//!

pub trait Operator {
    fn set_a(&mut self, a: i32);
    fn set_b(&mut self, b: i32);
    fn result(&self) -> i32;
}

pub trait OperatorFactory {
    fn create(&self) -> Box<dyn Operator>;
}

#[derive(Default)]
pub struct PlusOperator {
    a: i32,
    b: i32,
}

impl Operator for PlusOperator {
    fn set_a(&mut self, a: i32) {
        self.a = a;
    }
    fn set_b(&mut self, b: i32) {
        self.b = b;
    }
    fn result(&self) -> i32 {
        self.a + self.b
    }
}

pub struct PlusOperatorFactory {}

impl OperatorFactory for PlusOperatorFactory {
    fn create(&self) -> Box<dyn Operator> {
        Box::new(PlusOperator::default())
    }
}

#[derive(Default)]
pub struct MinusOperator {
    a: i32,
    b: i32,
}

impl Operator for MinusOperator {
    fn set_a(&mut self, a: i32) {
        self.a = a;
    }
    fn set_b(&mut self, b: i32) {
        self.b = b;
    }
    fn result(&self) -> i32 {
        self.a - self.b
    }
}

pub struct MinusOperatorFactory {}

impl OperatorFactory for MinusOperatorFactory {
    fn create(&self) -> Box<dyn Operator> {
        Box::new(MinusOperator::default())
    }
}

pub fn compute(factory: impl OperatorFactory, a: i32, b: i32) -> i32 {
    let mut op = factory.create();
    op.set_a(a);
    op.set_b(b);
    op.result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let factory = PlusOperatorFactory {};
        assert_eq!(3, compute(factory, 1, 2));
        let factory = MinusOperatorFactory {};
        assert_eq!(2, compute(factory, 4, 2));
    }
}