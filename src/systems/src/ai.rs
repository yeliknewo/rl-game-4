use dependencies::specs::{System, RunArg};
use event::{FrontChannel, BackChannel};
use event_enums::ai_x_control::{AiToControl, AiFromControl};
use event_enums::feeder_x_ai::{FeederToAi, FeederFromAi};
use utils::{Delta, Player};

pub struct AiSystem {
    feeder_back_channel: BackChannel<FeederToAi, FeederFromAi>,
    control_front_channel: FrontChannel<AiToControl, AiFromControl>,
}

impl AiSystem {
    pub fn new(
        feeder_back_channel: BackChannel<FeederToAi, FeederFromAi>,
        control_front_channel: FrontChannel<AiToControl, AiFromControl>,
    ) -> AiSystem {
        AiSystem {
            feeder_back_channel: feeder_back_channel,
            control_front_channel: control_front_channel,
        }
    }

    fn process_event(&mut self, event: FeederToAi) {
        match event {
            FeederToAi::PlayerPosition(player, position) => self.control_front_channel.send_to(AiToControl::Right(1.0, player)),
        }
    }
}

impl System<Delta> for AiSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {

        while let Some(event) = self.feeder_back_channel.try_recv_to() {
            self.process_event(event);
        }

        arg.fetch(|_| ());
    }
}
