use dependencies::specs::{System, RunArg};
use event::{FrontChannel, BackChannel};
use event_enums::feeder_x_ai::{FeederToAi, FeederToAi};
use utils::{Delta};

pub struct FeederSystem {
    ai_front_channel: FrontChannel<FeederToAi, FeederToAi>,
}

impl AiSystem {
    pub fn new(
        control_front_channel: FrontChannel<AiToControl, AiFromControl>
    ) -> AiSystem {
        AiSystem {
            control_front_channel: control_front_channel,
        }
    }
}

impl System<Delta> for AiSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
