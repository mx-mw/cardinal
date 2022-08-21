mod bin;
mod two;

use macros::Component;

pub trait Component {
    fn init(&mut self); // Called just before the first frame
    fn update(&mut self) {} // Called at the beginning of each frame (before rendering)
    fn timer(&mut self) {} // Called at an interval (cardinal::TIMER_INTERVAL)
    fn render(&mut self); // Called at the end of each frame (after rendering)
}

#[derive(Component)]
pub struct None;

pub struct Bin<T: Component>(pub T);

impl<T: Component> Component for Bin<T> {
    fn init(&mut self) {
        self.0.init();
    }

    fn update(&mut self) {
        self.0.update();
    }

    fn timer(&mut self) {
        self.0.timer();
    }

    fn render(&mut self) {
        self.0.render();
    }
}
