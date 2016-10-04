#[macro_use]
extern crate log;
#[macro_use]
extern crate gfx;

extern crate dependencies;
extern crate utils;

pub mod pipeline;
pub mod rl_glutin;
pub mod rl_sdl2;
pub mod shaders;

pub use pipeline::{Bundle, make_shaders, Packet, pipe, ProjectionData, TextureData};
pub use shaders::{Shaders};

pub type GlDevice = dependencies::gfx_device_gl::Device;
pub type GlFactory = dependencies::gfx_device_gl::Factory;
pub type GlResources = dependencies::gfx_device_gl::Resources;
pub type GlCommandBuffer = dependencies::gfx_device_gl::CommandBuffer;
pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type OutColor = gfx::handle::RenderTargetView<GlResources, ColorFormat>;
pub type OutDepth = gfx::handle::DepthStencilView<GlResources, DepthFormat>;
pub type GlEncoder = gfx::Encoder<GlResources, GlCommandBuffer>;
pub type GlTexture = gfx::handle::ShaderResourceView<GlResources, [f32; 4]>;
pub type WindowSettings<'a> = (&'a str, u32, u32);

pub struct GfxWindow<W, T> {
    out_color: OutColor,
    out_depth: OutDepth,
    device: GlDevice,
    factory: GlFactory,
    window: W,
    extras: T,
}

impl<W, T> GfxWindow<W, T> {
    pub fn new(
        out_color: OutColor,
        out_depth: OutDepth,
        device: GlDevice,
        factory: GlFactory,
        window: W,
        extras: T,
    ) -> GfxWindow<W, T> {
        GfxWindow {
            out_color: out_color,
            out_depth: out_depth,
            device: device,
            factory: factory,
            window: window,
            extras: extras,
        }
    }

    pub fn get_out_color(&self) -> &OutColor {
        &self.out_color
    }

    pub fn get_out_depth(&self) -> &OutDepth {
        &self.out_depth
    }

    pub fn get_device(&self) -> &GlDevice {
        &self.device
    }

    pub fn get_factory(&self) -> &GlFactory {
        &self.factory
    }

    pub fn get_window(&self) -> &W {
        &self.window
    }

    pub fn get_extras(&self) -> &T {
        &self.extras
    }

    pub fn get_mut_device(&mut self) -> &mut GlDevice {
        &mut self.device
    }

    pub fn get_mut_factory(&mut self) -> &mut GlFactory {
        &mut self.factory
    }

    pub fn get_mut_window(&mut self) -> &mut W {
        &mut self.window
    }

    pub fn get_mut_extras(&mut self) -> &mut T {
        &mut self.extras
    }
}
