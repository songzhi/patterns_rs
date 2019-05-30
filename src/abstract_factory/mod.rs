pub mod xml;
pub mod rdb;

/// 订单主记录
pub trait OrderMainDAO {
    fn save_order_main(&self) -> String;
}

/// 订单详情记录
pub trait OrderDetailDAO {
    fn save_order_detail(&self) -> String;
}

/// 抽象模式工厂接口
pub trait DAOFactory {
    fn create_order_main_dao(&self) -> Box<dyn OrderMainDAO>;
    fn create_order_detail_dao(&self) -> Box<dyn OrderDetailDAO>;
}

pub fn get_main_and_detail(factory: Box<dyn DAOFactory>) -> (String, String) {
    (factory.create_order_main_dao().save_order_main(),
     factory.create_order_detail_dao().save_order_detail())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdb() {
        let factory = Box::new(rdb::RDBDAOFactory {});
        assert_eq!(("rdb main saved".to_string(), "rdb detail saved".to_string()), get_main_and_detail(factory))
    }

    #[test]
    fn test_xml() {
        let factory = Box::new(xml::XMLDAOFactory {});
        assert_eq!(("xml main saved".to_string(), "xml detail saved".to_string()), get_main_and_detail(factory))
    }
}