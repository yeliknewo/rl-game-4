use components::{Transform, CompPlayer};
use dependencies::cgmath::{Vector3, MetricSpace, dot};
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
                ScoreToFeeder::Lose(loser) => self.ai_front_channel.send_to(FeederToAi::RewardAndEnd({
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

        let mut players_and_positions = vec!();

        for (transform, player) in (&transforms, &players).iter() {
            players_and_positions.push((player.get_player(), transform.get_pos()));
        }

        self.ai_front_channel.send_to(FeederToAi::Reward(players_and_positions.iter().filter_map(|me| {
            let mut other = players_and_positions.iter().filter_map(|other| {
                if me.0 != other.0 && me.1 != other.1 {
                    Some(other.1)
                } else {
                    None
                }
            }).collect::<Vec<Vector3<Coord>>>();
            if other.len() == 1 {
                match me.0 {
                    Player::One => {
                        Some((me.0, -me.1.distance(other.pop().unwrap_or_else(|| panic!("Shit Happened"))).round() as i64))
                    },
                    Player::Two => {
                        Some((me.0, me.1.distance(other.pop().unwrap_or_else(|| panic!("Shit Happened"))).round() as i64))
                    }
                }
            } else {
                None
            }
        }).collect()));

        let players = vec!(Player::One, Player::Two);

        for player in players {
            let world_state: Vec<f64> = match player {
                Player::One => {
                    let p1_pos = players_and_positions[0].1;
                    let p2_pos = players_and_positions[1].1;
                    let dot = dot(p1_pos, p2_pos);
                    let mag = p1_pos.distance(p2_pos);

                    vec!(dot as f64, mag as f64)
                },
                Player::Two => {
                    let p1_pos = players_and_positions[0].1;
                    let p2_pos = players_and_positions[1].1;
                    let dot = dot(p2_pos, p1_pos);
                    let mag = p1_pos.distance(p2_pos);

                    vec!(dot as f64, mag as f64)
                },
            };

            self.ai_front_channel.send_to(FeederToAi::WorldState(player, world_state));
        }

    }
}
