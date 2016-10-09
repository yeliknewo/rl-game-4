use components::{Transform, CompPlayer, CompMoving};
use dependencies::cgmath::{Vector3, MetricSpace, dot};
use dependencies::specs::{System, RunArg, Join};
use event::{FrontChannel, BackChannel};
use event_enums::feeder_x_ai::{FeederToAi, FeederFromAi};
use event_enums::score_x_feeder::{ScoreToFeeder, ScoreFromFeeder};
use utils::{Delta, Coord, Player};

pub struct FeederSystem {
    ai_front_channel: FrontChannel<FeederToAi, FeederFromAi>,
    score_back_channel: BackChannel<ScoreToFeeder, ScoreFromFeeder>,
    time: f64,
}

impl FeederSystem {
    pub fn new(
        ai_front_channel: FrontChannel<FeederToAi, FeederFromAi>,
        score_back_channel: BackChannel<ScoreToFeeder, ScoreFromFeeder>,
    ) -> FeederSystem {
        FeederSystem {
            ai_front_channel: ai_front_channel,
            score_back_channel: score_back_channel,
            time: 0.0,
        }
    }
}

impl System<Delta> for FeederSystem {
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        self.time += delta_time;

        let (transforms, players, movings) = arg.fetch(|w| (
            w.read::<Transform>(),
            w.read::<CompPlayer>(),
            w.read::<CompMoving>(),
        ));

        if let Some(event) = self.score_back_channel.try_recv_to() {
            match event {
                ScoreToFeeder::Lose(loser) => self.ai_front_channel.send_to(FeederToAi::RewardAndEnd({
                    match loser {
                        Player::One => {
                            vec!((Player::One, -10000), (Player::Two, 10000))
                        },
                        Player::Two => {
                            vec!((Player::One, 10000), (Player::Two, -10000))
                        },
                    }
                })),
                ScoreToFeeder::LoseBoth => self.ai_front_channel.send_to(FeederToAi::RewardAndEnd({
                    vec!((Player::One, -10000), (Player::Two, -10000))
                })),
            }
        }

        let mut player_data = vec!();

        for (transform, player, moving) in (&transforms, &players, &movings).iter() {
            player_data.push((player.get_player(), transform.get_pos(), moving.get_velocity()));
        }

        self.ai_front_channel.send_to(FeederToAi::Reward(player_data.iter().filter_map(|me| {
            let other = player_data.iter().filter_map(|other| {
                if me.0 != other.0 && me.1 != other.1 {
                    Some(other.1)
                } else {
                    None
                }
            }).collect::<Vec<Vector3<Coord>>>();
            if other.len() == 1 {
                match me.0 {
                    Player::One => {
                        Some((me.0, 25 - me.1.distance(Vector3::new(0.0, 0.0, 0.0)).round() as i64))
                    },
                    Player::Two => {
                        Some((me.0, 25 - me.1.distance(Vector3::new(0.0, 0.0, 0.0)).round() as i64))
                    }
                }
            } else {
                None
            }
        }).collect()));

        let players = vec!(Player::One, Player::Two);

        let center = Vector3::new(0.0, 0.0, 0.0);

        for player in players {
            let world_state: Vec<f64> = match player {
                Player::One => {
                    let p1_pos = player_data[0].1;
                    let p2_pos = player_data[1].1;
                    let vel = player_data[0].2;
                    let dot1 = dot(p1_pos, p2_pos);
                    let mag = p1_pos.distance(p2_pos);
                    let dot2 = dot(p1_pos, center);
                    let mag2 = p1_pos.distance(center);

                    vec!(dot1 as f64, mag as f64, dot2 as f64, mag2 as f64, vel.x as f64, vel.y as f64, self.time)
                },
                Player::Two => {
                    let p1_pos = player_data[0].1;
                    let p2_pos = player_data[1].1;
                    let vel = player_data[1].2;
                    let dot1 = dot(p2_pos, p1_pos);
                    let mag = p1_pos.distance(p2_pos);
                    let dot2 = dot(p2_pos, center);
                    let mag2 = p2_pos.distance(center);

                    vec!(dot1 as f64, mag as f64, dot2 as f64, mag2 as f64, vel.x as f64, vel.y as f64, self.time)
                },
            };

            self.ai_front_channel.send_to(FeederToAi::WorldState(player, world_state));
        }

    }
}
