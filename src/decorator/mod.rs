//! # 装饰模式
//!
//! 装饰模式使用对象组合的方式动态改变或增加对象行为。
//!

pub trait Component {
    fn eval(&self) -> i64;
}

pub struct ConcreteComponent {
    num: i64
}

impl Component for ConcreteComponent {
    fn eval(&self) -> i64 {
        self.num
    }
}

pub struct MulDecorator {
    component: Box<dyn Component>,
    num: i64,
}

impl MulDecorator {
    pub fn wrap(component: Box<dyn Component>, num: i64) -> Box<dyn Component> {
        Box::new(Self {
            component,
            num,
        })
    }
}

impl Component for MulDecorator {
    fn eval(&self) -> i64 {
        self.num * self.component.eval()
    }
}

pub struct AddDecorator {
    component: Box<dyn Component>,
    num: i64,
}

impl Component for AddDecorator {
    fn eval(&self) -> i64 {
        self.num + self.component.eval()
    }
}

impl AddDecorator {
    pub fn wrap(component: Box<dyn Component>, num: i64) -> Box<dyn Component> {
        Box::new(Self {
            component,
            num,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c: Box<dyn Component> = Box::new(ConcreteComponent { num: 3 });
        c = MulDecorator::wrap(c, 10);
        c = AddDecorator::wrap(c, 3);
        assert_eq!(c.eval(), 33);
    }
}