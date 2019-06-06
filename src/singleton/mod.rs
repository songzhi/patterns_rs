//! # 单例模式

use once_cell::sync::OnceCell;

pub struct Singleton {}

static SINGLETON: OnceCell<Singleton> = OnceCell::new();

pub fn get_instance() -> &'static Singleton {
    SINGLETON.get_or_init(|| Singleton {})
}