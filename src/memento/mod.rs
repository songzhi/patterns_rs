//! # 备忘录模式
//!
//! 备忘录模式用于保存程序内部状态到外部，又不希望暴露内部状态的情形。
//!
//! 程序内部状态使用窄接口传递给外部进行存储，从而不暴露程序实现细节。
//!
//! 备忘录模式同时可以离线保存内部状态，如保存到数据库，文件等。


use downcast_rs::Downcast;


pub trait Memento: Downcast {}
impl_downcast!(Memento);
#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    hp: isize,
    mp: isize,
}

pub struct GameMemento {
    hp: isize,
    mp: isize,
}

impl Memento for GameMemento {}

impl Game {
    pub fn play(&mut self, hp_delta: isize, mp_delta: isize) {
        self.hp += hp_delta;
        self.mp += mp_delta;
    }
    pub fn save(&self) -> Box<dyn Memento> {
        Box::new(GameMemento {
            hp: self.hp,
            mp: self.mp,
        })
    }
    pub fn load(memento: Box<dyn Memento>) -> Option<Self> {
        let memento = memento.downcast_ref::<GameMemento>()?;
        Some(Self {
            hp: memento.hp,
            mp: memento.mp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut game = Game {
            hp: 32,
            mp: 64,
        };
        game.play(10, 20);
        let memento = game.save();
        let loaded_game = Game::load(memento).unwrap();
        assert_eq!(game, loaded_game);
    }
}