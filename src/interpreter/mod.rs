//! # 解释器模式
//!
//! 解释器模式定义一套语言文法，并设计该语言解释器，使用户能使用特定文法控制解释器行为。
//!
//! 解释器模式的意义在于，它分离多种复杂功能的实现，每个功能只需关注自身的解释。
//!
//! 对于调用者不用关心内部的解释器的工作，只需要用简单的方式组合命令就可以。

pub trait Node {
    fn interpret(&self) -> i64;
}

pub struct ValNode {
    val: i64
}

impl Node for ValNode {
    fn interpret(&self) -> i64 {
        self.val
    }
}

pub struct AddNode {
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}

impl Node for AddNode {
    fn interpret(&self) -> i64 {
        self.left.interpret() + self.right.interpret()
    }
}

pub struct MinusNode {
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}

impl Node for MinusNode {
    fn interpret(&self) -> i64 {
        self.left.interpret() - self.right.interpret()
    }
}

#[derive(Default)]
pub struct Parser<'a> {
    tokens: Vec<&'a str>,
    pos: usize,
    prev: Option<Box<dyn Node>>,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            tokens: buffer.split(' ').collect(),
            pos: 0,
            prev: None,
        }
    }
    pub fn parse(mut self) -> Option<Box<dyn Node>> {
        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                "+" => self.prev = self.new_add_node(),
                "-" => self.prev = self.new_minus_node(),
                _ => self.prev = self.new_val_node()
            }
        }
        self.prev
    }
    fn new_add_node(&mut self) -> Option<Box<dyn Node>> {
        self.pos += 1;
        Some(Box::new(AddNode {
            left: self.prev.take()?,
            right: self.new_val_node()?,
        }))
    }
    fn new_minus_node(&mut self) -> Option<Box<dyn Node>> {
        self.pos += 1;
        Some(Box::new(MinusNode {
            left: self.prev.take()?,
            right: self.new_val_node()?,
        }))
    }
    fn new_val_node(&mut self) -> Option<Box<dyn Node>> {
        let val = self.tokens[self.pos].parse::<i64>().ok()?;
        self.pos += 1;
        Some(Box::new(ValNode {
            val
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let parser = Parser::new("1 + 2 + 3 - 4 + 5 - 6");
        assert_eq!(parser.parse().unwrap().interpret(), 1);
    }
}