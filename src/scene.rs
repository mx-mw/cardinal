use crate::{component::{Component, ComponentRequirements}, CardinalResult};

pub struct Scene {
	children: Vec<Box<dyn Component>>,
}

pub fn scene(children: Vec<Box<dyn Component>>) -> Scene {
	Scene {
		children
	}
}

impl Component for Scene {
	fn init(
		&mut self,
		surface_config: &wgpu::SurfaceConfiguration,
		adapter: &wgpu::Adapter,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> CardinalResult<()> {
		for i in &mut self.children {
			i.init(surface_config, adapter, device, queue)?;
		}
		Ok(())
	}
}

impl ComponentRequirements for Scene {
	
}