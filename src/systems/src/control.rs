use dependencies::specs::{System, RunArg};
use event::{FrontChannel, BackChannel};
use event_enums::main_x_control::{MainToControl, MainFromControl};
use event_enums::control_x_player::{ControlToPlayer, ControlFromPlayer};
use utils::{Delta};

#[derive(Debug)]
pub struct ControlSystem {
    main_back_channel: BackChannel<MainToControl, MainFromControl>,
    player_front_channel: FrontChannel<ControlToPlayer, ControlFromPlayer>,
}

impl ControlSystem {
    pub fn new(
        main_back_channel: BackChannel<MainToControl, MainFromControl>,
        player_front_channel: FrontChannel<ControlToPlayer, ControlFromPlayer>
    ) -> ControlSystem {
        ControlSystem {
            main_back_channel: main_back_channel,
            player_front_channel: player_front_channel,
        }
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
