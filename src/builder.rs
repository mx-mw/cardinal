use std::time::{Duration, Instant};
use winit::{
	event::{self, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

use crate::{scene, CardinalResult, Component, ComponentRequirements, Scene, Spawner};

pub async fn start(title: &str, layout: Vec<Box<dyn Component>>) -> CardinalResult<()> {
	let event_loop = EventLoop::new();
	let mut builder = winit::window::WindowBuilder::new();
	builder = builder.with_title(title);

	let window = builder.build(&event_loop).unwrap();

	let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);

	let instance = wgpu::Instance::new(backend);
	let (size, surface) = unsafe {
		let size = window.inner_size();
		let surface = instance.create_surface(&window);
		(size, surface)
	};
	let adapter =
		wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
			.await
			.expect("No suitable GPU adapters found on the system!");

	let optional_features = Scene::optional_features();
	let required_features = Scene::required_features();
	let adapter_features = adapter.features();

	let required_downlevel_capabilities = Scene::required_downlevel_capabilities();
	let downlevel_capabilities = adapter.get_downlevel_properties();
	assert!(
		downlevel_capabilities.shader_model >= required_downlevel_capabilities.shader_model,
		"Adapter does not support the minimum shader model required to run this scene: {:?}",
		required_downlevel_capabilities.shader_model
	);
	assert!(
		downlevel_capabilities
			.flags
			.contains(required_downlevel_capabilities.flags),
		"Adapter does not support the downlevel capabilities required to run this scene: {:?}",
		required_downlevel_capabilities.flags - downlevel_capabilities.flags
	);

	// Make sure we use the texture resolution limits from the adapter, so we can support images the size of the surface.
	let needed_limits = Scene::required_limits().using_resolution(adapter.limits());

	let trace_dir = std::env::var("WGPU_TRACE");
	let (device, queue) = adapter
		.request_device(
			&wgpu::DeviceDescriptor {
				label: None,
				features: (optional_features & adapter_features) | required_features,
				limits: needed_limits,
			},
			trace_dir.ok().as_ref().map(std::path::Path::new),
		)
		.await
		.expect("Unable to find a suitable GPU adapter!");
	let spawner = Spawner::new();
	let mut config = wgpu::SurfaceConfiguration {
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
		format: surface.get_preferred_format(&adapter).unwrap(),
		width: size.width,
		height: size.height,
		present_mode: wgpu::PresentMode::Mailbox,
	};
	surface.configure(&device, &config);
	let mut scene = scene(layout);
	scene.init(&config, &adapter, &device, &queue)?;

	#[cfg(not(target_arch = "wasm32"))]
	let mut last_update_inst = Instant::now();
	#[cfg(not(target_arch = "wasm32"))]
	let mut last_frame_inst = Instant::now();
	#[cfg(not(target_arch = "wasm32"))]
	let (mut frame_count, mut accum_time) = (0, 0.0);

	event_loop.run(move |event, _, control_flow| {
		let _ = (&instance, &adapter); // force ownership by the closure
		*control_flow = if cfg!(feature = "metal-auto-capture") {
			ControlFlow::Exit
		} else {
			ControlFlow::Poll
		};
		match event {
			event::Event::RedrawEventsCleared => {
				#[cfg(not(target_arch = "wasm32"))]
				{
					// Clamp to some max framerate to avoid busy-looping too much
					// (we might be in wgpu::PresentMode::Mailbox, thus discarding superfluous frames)
					//
					// winit has window.current_monitor().video_modes() but that is a list of all full screen video modes.
					// So without extra dependencies it's a bit tricky to get the max refresh rate we can run the window on.
					// Therefore we just go with 60fps - sorry 120hz+ folks!
					let target_frametime = Duration::from_secs_f64(1.0 / 60.0);
					let time_since_last_frame = last_update_inst.elapsed();
					if time_since_last_frame >= target_frametime {
						window.request_redraw();
						last_update_inst = Instant::now();
					} else {
						*control_flow = ControlFlow::WaitUntil(
							Instant::now() + target_frametime - time_since_last_frame,
						);
					}

					spawner.run_until_stalled();
				}

				#[cfg(target_arch = "wasm32")]
				window.request_redraw();
			}
			event::Event::WindowEvent {
				event:
					WindowEvent::Resized(size)
					| WindowEvent::ScaleFactorChanged {
						new_inner_size: &mut size,
						..
					},
				..
			} => {
				config.width = size.width.max(1);
				config.height = size.height.max(1);
				scene.resize(&config, &device, &queue).unwrap();
				surface.configure(&device, &config);
			}
			event::Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput {
					input:
						event::KeyboardInput {
							virtual_keycode: Some(event::VirtualKeyCode::Escape),
							state: event::ElementState::Pressed,
							..
						},
					..
				}
				| WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
				}
				#[cfg(not(target_arch = "wasm32"))]
				WindowEvent::KeyboardInput {
					input:
						event::KeyboardInput {
							virtual_keycode: Some(event::VirtualKeyCode::R),
							state: event::ElementState::Pressed,
							..
						},
					..
				} => {
					println!("{:#?}", instance.generate_report());
				}
				_ => {
					scene.frame(event).unwrap();
				}
			},
			event::Event::RedrawRequested(_) => {
				#[cfg(not(target_arch = "wasm32"))]
				{
					accum_time += last_frame_inst.elapsed().as_secs_f32();
					last_frame_inst = Instant::now();
					frame_count += 1;
					if frame_count == 100 {
						println!(
							"Avg frame time {}ms",
							accum_time * 1000.0 / frame_count as f32
						);
						accum_time = 0.0;
						frame_count = 0;
					}
				}

				let frame = match surface.get_current_texture() {
					Ok(frame) => frame,
					Err(_) => {
						surface.configure(&device, &config);
						surface
							.get_current_texture()
							.expect("Failed to acquire next surface texture!")
					}
				};
				let view = frame
					.texture
					.create_view(&wgpu::TextureViewDescriptor::default());

				scene.render(&view, &device, &queue, &spawner).unwrap();

				frame.present();
			}
			_ => {}
		}
	});
}
