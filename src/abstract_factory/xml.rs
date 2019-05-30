use super::*;

/// XML存储
pub struct XMLMainDAO {}

impl OrderMainDAO for XMLMainDAO {
    fn save_order_main(&self) -> String {
        String::from("xml main saved")
    }
}

pub struct XMLDetailDAO {}

impl OrderDetailDAO for XMLDetailDAO {
    fn save_order_detail(&self) -> String {
        String::from("xml detail saved")
    }
}

pub struct XMLDAOFactory {}

impl DAOFactory for XMLDAOFactory {
    fn create_order_main_dao(&self) -> Box<dyn OrderMainDAO> {
        Box::new(XMLMainDAO {})
    }
    fn create_order_detail_dao(&self) -> Box<dyn OrderDetailDAO> {
        Box::new(XMLDetailDAO {})
    }
}