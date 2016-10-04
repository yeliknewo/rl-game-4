pub mod glutin {
    use graphics::{GfxWindow};
    use graphics::rl_glutin::{Window, Extras};
    use event_clump::{FrontEventClump};

    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        false
    }
}

pub mod sdl2 {
    use graphics::{GfxWindow};
    use graphics::rl_sdl2::{Window, Extras};
    use event_clump::{FrontEventClump};

    pub fn handle_events(gfx_window: &mut GfxWindow<Window, Extras>, front_event_clump: &mut FrontEventClump) -> bool {
        false
    }
}
