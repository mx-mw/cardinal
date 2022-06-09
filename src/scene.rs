use crate::{
	component::{Component, ComponentRequirements},
	CardinalResult, Spawner,
};

pub struct Scene {
	children: Vec<Box<dyn Component>>,
}

pub fn scene(children: Vec<Box<dyn Component>>) -> Scene {
	Scene { children }
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

	fn render(
		&mut self,
		view: &wgpu::TextureView,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		spawner: &Spawner,
	) -> CardinalResult<()> {
		let color_attachments = [wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: true,
            },
        }];
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: None,
        };

        // get command encoder
        let mut command_encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            // render pass
            let mut rpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            rpass.set_pipeline(&self.render_pipeline);
            // render dst particles
            rpass.set_vertex_buffer(0, self.particle_buffers[(self.frame_num + 1) % 2].slice(..));
            // the three instance-local vertices
            rpass.set_vertex_buffer(1, self.vertices_buffer.slice(..));
            rpass.draw(0..3, 0..NUM_PARTICLES);
        }
        command_encoder.pop_debug_group();

        // done
        queue.submit(Some(command_encoder.finish()));

		Ok(())
	}
}

impl ComponentRequirements for Scene {}
