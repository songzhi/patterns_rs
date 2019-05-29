//!# 外观模式
//!
//!API 为facade 模块的外观接口，大部分代码使用此接口简化对facade类的访问。
//
//!facade模块同时暴露了a和b 两个Module 的NewXXX和interface，其它代码如果需要使用细节功能时可以直接调用。
//!

pub trait Api {
    fn test(&self) -> String;
}

pub fn new_api() -> Box<dyn Api> {
    Box::new(ApiImpl {
        a: new_a_module_api(),
        b: new_b_module_api(),
    })
}

struct ApiImpl {
    a: Box<dyn AModuleApi>,
    b: Box<dyn BModuleApi>,
}

impl Api for ApiImpl {
    fn test(&self) -> String {
        format!("{}\n{}", self.a.test_a(), self.b.test_b())
    }
}

trait AModuleApi {
    fn test_a(&self) -> String;
}

fn new_a_module_api() -> Box<dyn AModuleApi> {
    Box::new(AModuleImpl {})
}

trait BModuleApi {
    fn test_b(&self) -> String;
}

fn new_b_module_api() -> Box<dyn BModuleApi> {
    Box::new(BModuleImpl {})
}

struct AModuleImpl {}

impl AModuleApi for AModuleImpl {
    fn test_a(&self) -> String {
        String::from("A module running")
    }
}

struct BModuleImpl {}

impl BModuleApi for BModuleImpl {
    fn test_b(&self) -> String {
        String::from("B module running")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let api = new_api();
        assert_eq!("A module running\nB module running", api.test().as_str())
    }
}