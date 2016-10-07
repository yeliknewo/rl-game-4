use event::{FrontChannel, BackChannel, two_way_channel};
use event_enums::main_x_control::{MainToControl, MainFromControl};
use event_enums::main_x_game::{MainToGame, MainFromGame};
use event_enums::main_x_render::{MainToRender, MainFromRender};

pub fn make_event_clumps() -> (FrontEventClump, BackEventClump) {
    let (front_control, back_control) = two_way_channel();
    let (front_render, back_render) = two_way_channel();
    let (front_game, back_game) = two_way_channel();

    let front_event_clump = FrontEventClump::new(
        front_render,
        front_control,
        front_game,
    );

    let back_event_clump = BackEventClump::new(
        back_render,
        back_control,
        back_game,
    );

    (front_event_clump, back_event_clump)
}

pub struct BackEventClump {
    render: Option<BackChannel<MainToRender, MainFromRender>>,
    control: Option<BackChannel<MainToControl, MainFromControl>>,
    game: Option<BackChannel<MainToGame, MainFromGame>>,
}

impl BackEventClump {
    fn new(
        render: BackChannel<MainToRender, MainFromRender>,
        control: BackChannel<MainToControl, MainFromControl>,
        game: BackChannel<MainToGame, MainFromGame>,
    ) -> BackEventClump {
        BackEventClump {
            render: Some(render),
            control: Some(control),
            game: Some(game),
        }
    }

    pub fn take_render(&mut self) -> Option<BackChannel<MainToRender, MainFromRender>> {
        self.render.take()
    }

    pub fn take_control(&mut self) -> Option<BackChannel<MainToControl, MainFromControl>> {
        self.control.take()
    }

    pub fn take_game(&mut self) -> Option<BackChannel<MainToGame, MainFromGame>> {
        self.game.take()
    }
}

pub struct FrontEventClump {
    render: Option<FrontChannel<MainToRender, MainFromRender>>,
    control: Option<FrontChannel<MainToControl, MainFromControl>>,
    game: Option<FrontChannel<MainToGame, MainFromGame>>,
}

impl FrontEventClump {
    fn new(
        render: FrontChannel<MainToRender, MainFromRender>,
        control: FrontChannel<MainToControl, MainFromControl>,
        game: FrontChannel<MainToGame, MainFromGame>,
    ) -> FrontEventClump {
        FrontEventClump {
            render: Some(render),
            control: Some(control),
            game: Some(game),
        }
    }

    // pub fn take_render(&mut self) -> Option<FrontChannel<ToRender, FromRender>> {
    //     self.render.take()
    // }
    //
    // pub fn take_control(&mut self) -> Option<FrontChannel<ToControl, FromControl>> {
    //     self.control.take()
    // }
    //
    // pub fn give_render(&mut self, render: FrontChannel<ToRender, FromRender>) {
    //     self.render = Some(render);
    // }
    //
    // pub fn give_control(&mut self, control: FrontChannel<ToControl, FromControl>) {
    //     self.control = Some(control);
    // }

    pub fn get_mut_render(&mut self) -> Option<&mut FrontChannel<MainToRender, MainFromRender>> {
        self.render.as_mut()
    }

    pub fn get_mut_control(&mut self) -> Option<&mut FrontChannel<MainToControl, MainFromControl>> {
        self.control.as_mut()
    }

    pub fn get_mut_game(&mut self) -> Option<&mut FrontChannel<MainToGame, MainFromGame>> {
        self.game.as_mut()
    }
}
