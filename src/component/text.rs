use crate::{Component, CardinalResult};

pub struct Text {
	text: String
}

pub fn text(text: &'static str) -> Box<dyn Component> {
	Box::new(Text {
		text: text.into()
	})
}

impl Component for Text {
	fn init(
		&mut self,
		surface_config: &wgpu::SurfaceConfiguration,
		adapter: &wgpu::Adapter,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> CardinalResult<()> {

		Ok(())
	}
}