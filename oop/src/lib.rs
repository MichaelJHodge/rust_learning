pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    //a trait object - a stand in for any type inside  box that implements
    //the Box trait.
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
