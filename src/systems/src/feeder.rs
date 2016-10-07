use dependencies::specs::{System, RunArg};
use event::{FrontChannel};
use event_enums::feeder_x_ai::{FeederToAi, FeederFromAi};
use utils::{Delta};

pub struct FeederSystem {
    ai_front_channel: FrontChannel<FeederToAi, FeederFromAi>,
}

impl FeederSystem {
    pub fn new(
        ai_front_channel: FrontChannel<FeederToAi, FeederFromAi>
    ) -> FeederSystem {
        FeederSystem {
            ai_front_channel: ai_front_channel,
        }
    }
}

impl System<Delta> for FeederSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
