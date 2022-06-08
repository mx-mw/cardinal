use crate::{component::Component, BaseMethods, CardinalResult, Spawner};
use winit::event::WindowEvent;

pub struct Scene<R: Component<()>> {
	children: R,
}

impl Component<()> for Scene<> {
	
}
