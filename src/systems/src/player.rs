use dependencies::specs::{System, RunArg};
use event::{BackChannel};
use event_enums::control_x_player::{ControlToPlayer, ControlFromPlayer};
use utils::{Delta};

pub struct PlayerSystem {
    control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>,
}

impl PlayerSystem {
    pub fn new(
        control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>
    ) -> PlayerSystem {
        PlayerSystem {
            control_back_channel: control_back_channel,
        }
    }
}

impl System<Delta> for PlayerSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
