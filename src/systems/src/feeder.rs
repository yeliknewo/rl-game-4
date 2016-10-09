use components::{Transform, CompPlayer};
use dependencies::cgmath::{Vector3, MetricSpace};
use dependencies::specs::{System, RunArg, Join};
use event::{FrontChannel, BackChannel};
use event_enums::feeder_x_ai::{FeederToAi, FeederFromAi};
use event_enums::score_x_feeder::{ScoreToFeeder, ScoreFromFeeder};
use utils::{Delta, Coord, Player};

pub struct FeederSystem {
    ai_front_channel: FrontChannel<FeederToAi, FeederFromAi>,
    score_back_channel: BackChannel<ScoreToFeeder, ScoreFromFeeder>,
}

impl FeederSystem {
    pub fn new(
        ai_front_channel: FrontChannel<FeederToAi, FeederFromAi>,
        score_back_channel: BackChannel<ScoreToFeeder, ScoreFromFeeder>,
    ) -> FeederSystem {
        FeederSystem {
            ai_front_channel: ai_front_channel,
            score_back_channel: score_back_channel,
        }
    }
}

impl System<Delta> for FeederSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let (transforms, players) = arg.fetch(|w| (
            w.read::<Transform>(),
            w.read::<CompPlayer>(),
        ));

        if let Some(event) = self.score_back_channel.try_recv_to() {
            match event {
                ScoreToFeeder::Lose(loser) => self.ai_front_channel.send_to(FeederToAi::End({
                    match loser {
                        Player::One => {
                            vec!((Player::One, -1000), (Player::Two, 1000))
                        },
                        Player::Two => {
                            vec!((Player::One, 1000), (Player::Two, -1000))
                        },
                    }
                })),
            }
        }

        let mut vec = vec!();

        for (transform, player) in (&transforms, &players).iter() {
            vec.push((player.get_player(), transform.get_pos()));
        }

        self.ai_front_channel.send_to(FeederToAi::Reward(vec.iter().filter_map(|me| {
            let mut other = vec.iter().filter_map(|other| {
                if me.0 != other.0 && me.1 != other.1 {
                    Some(other.1)
                } else {
                    None
                }
            }).collect::<Vec<Vector3<Coord>>>();
            if other.len() == 1 {
                Some((me.0, me.1.distance(other.pop().unwrap_or_else(|| panic!("Shit Happened"))).round() as i64))
            } else {
                None
            }
        }).collect()));

        self.ai_front_channel.send_to(FeederToAi::PlayerPosition(vec));
    }
}
