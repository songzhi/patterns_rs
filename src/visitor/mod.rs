//! # 访问者模式

// The data we will visit
mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }

    impl Expr {
        pub fn int_lit(num: i64) -> Self {
            Expr::IntLit(num)
        }
        pub fn add(left: Box<Expr>, right: Box<Expr>) -> Self {
            Expr::Add(left, right)
        }
        pub fn sub(left: Box<Expr>, right: Box<Expr>) -> Self {
            Expr::Sub(left, right)
        }
    }
}

// The abstract visitor
mod visit {
    use super::ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use visit::*;
use ast::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;

impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 { panic!() }
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expr = Expr::add(Box::new(Expr::int_lit(16)), Box::new(Expr::int_lit(14)));
        let expr = Expr::sub(Box::new(expr), Box::new(Expr::int_lit(16)));
        let mut interpreter = Interpreter;
        assert_eq!(14, interpreter.visit_expr(&expr));
    }
}