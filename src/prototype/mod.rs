pub fn prototype() {
    let prototype = Object::new();

    let mut obj = prototype.clone();
    obj.set_x(500);
    obj.set_y(300);

    println!("prototype: {:?}", prototype);
    println!("obj: {:?}", obj);
}

// Prototype trait has the trait bound(= requires Clone trait)
trait Prototype: Clone {
    fn set_x(&mut self, x: usize);
    fn set_y(&mut self, y: usize);
}

#[derive(Clone, Debug)]
struct Object {
    x: usize,
    y: usize,
}

impl Object {
    fn new() -> Self {
        Self {
            x: 100,
            y: 200,
        }
    }
}

impl Prototype for Object {
    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}