//! # 享元模式
//!
//! 享元模式从对象中剥离出不发生改变且多个实例需要的重复数据，独立出一个享元，使多个对象共享，从而节省内存以及减少对象数量。

use std::collections::HashMap;
use std::rc::Rc;
use rand;

#[derive(Default)]
pub struct ImageFlyweightFactory {
    map: HashMap<String, Rc<ImageFlyweight>>
}

impl ImageFlyweightFactory {
    pub fn get(&mut self, filename: &str) -> Rc<ImageFlyweight> {
        if !self.map.contains_key(filename) {
            self.map.insert(filename.to_string(), Rc::new(ImageFlyweight::new(filename)));
        }
        self.map.get(filename).unwrap().clone()
    }
}

pub struct ImageFlyweight {
    data: String
}

impl ImageFlyweight {
    pub fn new(filename: &str) -> Self {
        let data = format!("image data {}", rand::random::<u64>());
        Self {
            data
        }
    }
}

pub struct ImageViewer {
    image: Rc<ImageFlyweight>
}

impl ImageViewer {
    pub fn new(image: Rc<ImageFlyweight>) -> Self {
        Self {
            image
        }
    }
    pub fn display(&self) -> String {
        format!("Display: {}", self.image.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut factory = ImageFlyweightFactory::default();
        let viewer1 = ImageViewer::new(factory.get("test.jpg"));
        let viewer2 = ImageViewer::new(factory.get("test.jpg"));
        assert_eq!(viewer1.image.data, viewer2.image.data);
    }
}