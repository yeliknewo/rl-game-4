use components::{Camera, RenderData, RenderId, Transform};
use dependencies::specs::{Planner, World};
use dependencies::time::{precise_time_ns};
use graphics::{OutColor, OutDepth};
use math::{OrthographicHelper};
use systems::render::{RenderSystem};
use systems::control::{ControlSystem};
use utils::{Delta, FpsCounter};

use event_clump::{BackEventClump};

pub struct Game {
    planner: Planner<Delta>,
    last_time: u64,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new(
        mut back_event_clump: BackEventClump,
        ortho_helper: OrthographicHelper,
        out_color: OutColor,
        out_depth: OutDepth
    ) -> Game {
        let mut planner = {
            let mut world = World::new();

            world.register::<Camera>();
            world.register::<RenderData>();
            world.register::<RenderId>();
            world.register::<Transform>();

            Planner::<Delta>::new(world, 8)
        };

        let renderer = RenderSystem::new(back_event_clump.take_render().unwrap(), out_color, out_depth);



        planner.add_system(
            ControlSystem::new(back_event_clump.take_control().unwrap()),
            "control",
            30
        );

        planner.add_system(
            renderer,
            "renderer",
            10
        );

        Game {
            planner: planner,
            last_time: precise_time_ns(),
            fps_counter: FpsCounter::new(),
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        self.planner.dispatch(delta);
        self.fps_counter.frame(delta);

        true
    }
}
