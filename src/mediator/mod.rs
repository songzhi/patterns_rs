//!# 中介者模式
//!
//!中介者模式封装对象之间互交，使依赖变的简单，并且使复杂互交简单化，封装在中介者中。
//!
use std::sync::Once;


static mut MEDIATOR: Option<Mediator> = None;
static INIT: Once = Once::new();

pub fn get_mediator_instance() -> &'static mut Mediator {
    unsafe {
        INIT.call_once(|| {
            MEDIATOR = Some(Mediator::default())
        });
        MEDIATOR.as_mut().unwrap()
    }
}

#[derive(Default)]
pub struct CDDriver {
    data: String
}

impl CDDriver {
    pub fn read_data(&mut self) {
        self.data = "music,image".to_string();
        get_mediator_instance().changed(ChangedItem::Cd(self));
    }
}

#[derive(Default)]
pub struct CPU {
    video: String,
    sound: String,
}

impl CPU {
    pub fn process(&mut self, data: String) {
        let sp: Vec<&str> = data.split(',').collect();
        self.sound = sp[0].to_string();
        self.video = sp[1].to_string();
        get_mediator_instance().changed(ChangedItem::Cpu(self));
    }
}

#[derive(Default)]
pub struct VideoCard {
    data: String
}

impl VideoCard {
    pub fn display(&mut self, data: String) {
        self.data = data;
        get_mediator_instance().changed(ChangedItem::Video(self));
    }
}

#[derive(Default)]
pub struct SoundCard {
    data: String
}

impl SoundCard {
    pub fn play(&mut self, data: String) {
        self.data = data;
        get_mediator_instance().changed(ChangedItem::Sound(self));
    }
}

#[derive(Default)]
pub struct Mediator {
    cd: CDDriver,
    cpu: CPU,
    video: VideoCard,
    sound: SoundCard,
}

pub enum ChangedItem<'a> {
    Cd(&'a CDDriver),
    Cpu(&'a CPU),
    Video(&'a VideoCard),
    Sound(&'a SoundCard),
}

impl Mediator {
    pub fn changed(&mut self, changed_item: ChangedItem) {
        match changed_item {
            ChangedItem::Cd(cd) =>
                self.cpu.process(cd.data.clone()),
            ChangedItem::Cpu(cpu) => {
                self.sound.play(cpu.sound.clone());
                self.video.display(cpu.video.clone())
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mediator = get_mediator_instance();
        // trigger
        mediator.cd.read_data();
        assert_eq!(mediator.cd.data.as_str(), "music,image");
        assert_eq!(mediator.cpu.sound.as_str(), "music");
        assert_eq!(mediator.cpu.video.as_str(), "image");
        assert_eq!(mediator.video.data.as_str(), "image");
        assert_eq!(mediator.sound.data.as_str(), "music");
    }
}