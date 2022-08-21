use super::Component;

pub struct Two<T, V>(pub T, pub V)
where
    T: Component,
    V: Component;

impl<T, V> Component for Two<T, V> where
T: Component,
V: Component {
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
