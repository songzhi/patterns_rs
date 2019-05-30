use super::*;

/// 为关系型数据库的OrderMainDAO实现
pub struct RDBMainDAO {}

impl OrderMainDAO for RDBMainDAO {
    fn save_order_main(&self) -> String {
        String::from("rdb main saved")
    }
}

/// 为关系型数据库的OrderDetailDAO实现
pub struct RDBDetailDAO {}

impl OrderDetailDAO for RDBDetailDAO {
    fn save_order_detail(&self) -> String {
        String::from("rdb detail saved")
    }
}

pub struct RDBDAOFactory {}

impl DAOFactory for RDBDAOFactory {
    fn create_order_main_dao(&self) -> Box<dyn OrderMainDAO> {
        Box::new(RDBMainDAO {})
    }
    fn create_order_detail_dao(&self) -> Box<dyn OrderDetailDAO> {
        Box::new(RDBDetailDAO {})
    }
}