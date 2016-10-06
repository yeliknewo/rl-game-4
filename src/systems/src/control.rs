use dependencies::specs::{System, RunArg};
use event::{FrontChannel, BackChannel};
use event_enums::ai_x_control::{AiToControl, AiFromControl};
use event_enums::main_x_control::{MainToControl, MainFromControl};
use event_enums::control_x_player::{ControlToPlayer, ControlFromPlayer};
use utils::{Delta};

#[derive(Debug)]
pub struct ControlSystem {
    main_back_channel: BackChannel<MainToControl, MainFromControl>,
    ai_back_channel: BackChannel<AiToControl, AiFromControl>,
    player_front_channel: FrontChannel<ControlToPlayer, ControlFromPlayer>,
}

impl ControlSystem {
    pub fn new(
        main_back_channel: BackChannel<MainToControl, MainFromControl>,
        ai_back_channel: BackChannel<AiToControl, AiFromControl>,
        player_front_channel: FrontChannel<ControlToPlayer, ControlFromPlayer>,
    ) -> ControlSystem {
        ControlSystem {
            main_back_channel: main_back_channel,
            ai_back_channel: ai_back_channel,
            player_front_channel: player_front_channel,
        }
    }

    fn process_main_event(&mut self, event: MainToControl) {
        match event {
            MainToControl::Up(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Up(amount, player)),
            MainToControl::Down(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Down(amount, player)),
            MainToControl::Left(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Left(amount, player)),
            MainToControl::Right(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Right(amount, player)),
        }
    }

    fn process_ai_event(&mut self, event: AiToControl) {
        match event {
            AiToControl::Up(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Up(amount, player)),
            AiToControl::Down(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Down(amount, player)),
            AiToControl::Left(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Left(amount, player)),
            AiToControl::Right(amount, player) => self.player_front_channel.send_to(ControlToPlayer::Right(amount, player)),
        }
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let needs_fetch = true;

        while needs_fetch {
            if let Some(event) = self.main_back_channel.try_recv_to() {
                self.process_main_event(event);
            }

            if let Some(event) = self.ai_back_channel.try_recv_to() {
                self.process_ai_event(event);
            }
        }

        arg.fetch(|_| ());
    }
}
