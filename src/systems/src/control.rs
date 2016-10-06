use dependencies::specs::{System, RunArg};
use event::{BackChannel};
use event_enums::main_x_control::{MainToControl, MainFromControl};
use utils::{Delta};

#[derive(Debug)]
pub struct ControlSystem {
    back_channel: BackChannel<MainToControl, MainFromControl>,
}

impl ControlSystem {
    pub fn new(back_channel: BackChannel<MainToControl, MainFromControl>) -> ControlSystem {
        ControlSystem {
            back_channel: back_channel,
        }
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
