#[macro_use]
extern crate log;

extern crate art;
extern crate components;
extern crate dependencies;
extern crate event;
extern crate graphics;
extern crate math;
extern crate systems;
extern crate utils;

mod event_clump;
mod game;
mod handle_events;

use std::thread;

use dependencies::gfx::{Device};
use event::{two_way_channel};
use graphics::{GlEncoder};
use graphics::rl_sdl2::{build_window};
use math::{OrthographicHelper};
use systems::render::{ToRender, FromRender};
// use systems::control::{ToControl, FromControl};

use event_clump::{FrontEventClump, BackEventClump};
use game::{Game};

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
    let (control_event_core, control_event_game) = two_way_channel();

    let encoder: GlEncoder = gfx_window.get_mut_factory().create_command_buffer().into();

    render_event_core.send_to(ToRender::Encoder(encoder.clone_empty()));
    render_event_core.send_to(ToRender::Encoder(encoder));

    let mut front_event_clump = FrontEventClump::new(render_event_core, control_event_core);

    let back_event_clump = BackEventClump::new(render_event_game, control_event_game);

    let game = Game::new(
        back_event_clump,
        ortho_helper,
        gfx_window.get_out_color().clone(),
        gfx_window.get_out_depth().clone()
    );

    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() { }
    });

    'main: loop {
        if let Some(event) = front_event_clump.get_mut_render().unwrap().try_recv_from() {
            match event {
                FromRender::Encoder(mut encoder) => {
                    if handle_events::sdl2::handle_events(&mut gfx_window, &mut front_event_clump) {
                        break 'main;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render().unwrap().send_to(ToRender::Encoder(encoder));
                    gfx_window.get_mut_window().gl_swap_window();
                    gfx_window.get_mut_device().cleanup();
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_control().unwrap().try_recv_from() {
            match event {

            }
        }
    }

    // game_handle.join().unwrap_or_else(|err| panic!("Error: {:?}", err));
}
