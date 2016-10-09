#[macro_use]
extern crate log;

extern crate art;
extern crate components;
extern crate dependencies;
extern crate event;
extern crate event_enums;
extern crate graphics;
extern crate math;
extern crate systems;
extern crate utils;

mod event_clump;
mod game;
mod handle_events;

use std::thread;

use dependencies::gfx::{Device};
use event_enums::main_x_render::{MainToRender, MainFromRender};
use event_enums::main_x_game::{MainToGame};
use graphics::{GlEncoder};
use graphics::rl_sdl2::{build_window};
use math::{OrthographicHelper};

use event_clump::{make_event_clumps};
use game::{Game};

pub fn start() {
    warn!("Starting Core Start");

    let (mut front_event_clump, back_event_clump) = make_event_clumps();

    let (width, height): (u32, u32) = (640, 640);

    let title = "rl-game-3";

    let left = -10.0;
    let right = 10.0;

    let near = 0.0;
    let far = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    // warn!("Making Window");
    let mut gfx_window = build_window((title, width, height));

    // warn!("Making Encoder");
    let encoder: GlEncoder = gfx_window.get_mut_factory().create_command_buffer().into();

    {
        let mut render_event_core = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none"));

        // warn!("Sending Empty Encoder");
        render_event_core.send_to(MainToRender::Encoder(encoder.clone_empty()));
        // warn!("Sending Encoder");
        render_event_core.send_to(MainToRender::Encoder(encoder));
    }

    let out_color = gfx_window.get_out_color().clone();
    let out_depth = gfx_window.get_out_depth().clone();

    // warn!("Making Game");
    let game = Game::new(
        gfx_window.get_mut_factory(),
        back_event_clump,
        ortho_helper,
        out_color,
        out_depth,
    );

    // warn!("Making Game Thread");
    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() { }
    });

    'main: loop {
        // warn!("Main Loop");
        if let Some(event) = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).try_recv_from() {
            match event {
                MainFromRender::Encoder(mut encoder) => {
                    if handle_events::sdl2::handle_events(&mut gfx_window, &mut front_event_clump) {
                        front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send_to(MainToRender::Encoder(encoder));
                        break 'main;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send_to(MainToRender::Encoder(encoder));
                    gfx_window.get_mut_window().gl_swap_window();
                    gfx_window.get_mut_device().cleanup();
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).try_recv_from() {
            match event {

            }
        }

        if let Some(event) = front_event_clump.get_mut_game().unwrap_or_else(|| panic!("Game was none")).try_recv_from() {
            match event {

            }
        }
    }

    front_event_clump.get_mut_game().unwrap_or_else(|| panic!("Game was none")).send_to(MainToGame::Exit);

    game_handle.join().unwrap_or_else(|err| panic!("Error: {:?}", err));
}
