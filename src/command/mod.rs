//!# 命令模式
//!
//!命令模式本质是把某个对象的方法调用封装到对象中，方便传递、存储、调用。
//!
use std::rc::Rc;

pub trait Command {
    fn execute(&self);
}

struct StartCommand {
    mb: Rc<MotherBoard>
}

impl Command for StartCommand {
    fn execute(&self) {
        self.mb.start()
    }
}

struct RebootCommand {
    mb: Rc<MotherBoard>
}

impl Command for RebootCommand {
    fn execute(&self) {
        self.mb.reboot()
    }
}

struct MotherBoard {}

impl MotherBoard {
    pub fn start(&self) {
        println!("system starting")
    }
    pub fn reboot(&self) {
        println!("system rebooting")
    }
}

pub struct Case {
    button1: Box<dyn Command>,
    button2: Box<dyn Command>,
}

impl Case {
    pub fn press_button1(&self) {
        self.button1.execute();
    }
    pub fn press_button2(&self) {
        self.button2.execute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mb = Rc::new(MotherBoard {});
        let button1 = Box::new(StartCommand { mb: mb.clone() });
        let button2 = Box::new(RebootCommand { mb: mb.clone() });
        let case = Case {
            button1,
            button2,
        };
        case.press_button1(); // system starting
        case.press_button2(); // system rebooting
    }
}