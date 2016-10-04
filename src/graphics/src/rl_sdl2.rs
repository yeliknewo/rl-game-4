use dependencies::sdl2::video::{GLContext};
use dependencies::sdl2;
use dependencies::gfx_window_sdl;

use super::{WindowSettings, GfxWindow};

pub type Window = ::dependencies::sdl2::video::Window;
pub type Extras = GLContext;

pub fn build_window(window_settings: WindowSettings) -> GfxWindow<Window, Extras> {
    let sdl = sdl2::init().unwrap_or_else(|err| panic!("Error while sdl2::init: {:?}", err));

    let video = sdl.video().unwrap_or_else(|err| panic!("Error while making sdl.video(): {:?}", err));

    let gl_attr = video.gl_attr();
    gl_attr.set_context_version(3, 2);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    let (title, width, height) = window_settings;

    let mut builder = video.window(title, width, height);
    let (window, context, device, factory, out_color, out_depth) = gfx_window_sdl::init(&mut builder);

    GfxWindow::new(out_color, out_depth, device, factory, window, context)
}
