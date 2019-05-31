//! # 单例模式

use std::sync::Once;

pub struct Singleton {}

static mut SINGLETON: Option<Singleton> = None;
static INIT: Once = Once::new();

pub fn get_instance() -> &'static Singleton {
    unsafe {
        INIT.call_once(|| {
            SINGLETON = Some(Singleton {})
        });
        SINGLETON.as_ref().unwrap()
    }
}