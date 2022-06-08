use crate::{CardinalResult, Spawner};
use winit::event::WindowEvent;

pub trait Component<T: Clone>: Sized {
	fn optional_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities {
            flags: wgpu::DownlevelFlags::empty(),
            shader_model: wgpu::ShaderModel::Sm5,
            ..wgpu::DownlevelCapabilities::default()
        }
    }
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::downlevel_webgl2_defaults() // These downlevel limits will allow the code to run on all possible hardware
    }
    fn init(
        _: &wgpu::SurfaceConfiguration,
        _: &wgpu::Adapter,
        _: &wgpu::Device,
        _: &wgpu::Queue,
		_: &'static T
    ) -> CardinalResult<Self>;
    fn resize(
        &mut self,
        _: &wgpu::SurfaceConfiguration,
        _: &wgpu::Device,
        _: &wgpu::Queue,
    ) -> CardinalResult<()> {Ok(())}
	fn tick(&mut self) -> CardinalResult<()> {Ok(())}
    fn frame(&mut self, _: WindowEvent) -> CardinalResult<()> {Ok(())}
    fn render(
        &mut self,
        _: &wgpu::TextureView,
        _: &wgpu::Device,
        _: &wgpu::Queue,
        _: &Spawner,
    ) -> CardinalResult<()> {Ok(())}
}