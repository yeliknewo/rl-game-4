#[macro_use]
extern crate log;

extern crate event;
extern crate graphics;
extern crate math;
extern crate systems;

use event::{two_way_channel};
use graphics::{GlEncoder};
use graphics::rl_sdl2::{build_window};
use math::{OrthographicHelper};
use systems::render::{ToRender, FromRender};
use systems::control::{ToControl, FromControl};

pub fn start() {
    warn!("Starting Core Start");

    let (width, height): (u32, u32) = (640, 480);

    let title = "rl-game-3";

    let left = -10.0;
    let right = 10.0;

    let near = 0.0;
    let far = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    let mut gfx_window = build_window((title, width, height));

    let (mut render_event_core, render_event_game) = two_way_channel();
    let (mut control_event_core, control_event_game) = two_way_channel();

    let encoder: GlEncoder = gfx_window.get_mut_factory().create_command_buffer().into();

    render_event_core.send_to(ToRender::Encoder(encoder.clone_empty()));
    render_event_core.send_to(ToRender::Encoder(encoder));
}
