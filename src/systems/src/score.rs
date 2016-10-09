use components::{Transform, CompPlayer, CompMoving};
use dependencies::cgmath::{Vector3};
use dependencies::specs::{System, RunArg, Join};
use event::{FrontChannel};
use event_enums::score_x_feeder::{ScoreToFeeder, ScoreFromFeeder};
use utils::{Delta, Player, Coord};

pub const PLAYER_ONE_START: Vector3<Coord> = Vector3 {
    x: -1.0,
    y: 0.0,
    z: 0.0,
};

pub const PLAYER_TWO_START: Vector3<Coord> = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const STARTING_VELOCITY: Vector3<Coord> = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub struct ScoreSystem {
    feeder_front_channel: FrontChannel<ScoreToFeeder, ScoreFromFeeder>,
    time: f64,
}

impl ScoreSystem {
    pub fn new(
        feeder_front_channel: FrontChannel<ScoreToFeeder, ScoreFromFeeder>,
    ) -> ScoreSystem {
        ScoreSystem {
            feeder_front_channel: feeder_front_channel,
            time: 0.0,
        }
    }
}

impl System<Delta> for ScoreSystem {
    fn run(&mut self, args: RunArg, delta_time: Delta) {
        self.time += delta_time;

        let (players, mut transforms, mut movings) = args.fetch(|w|(
                w.read::<CompPlayer>(),
                w.write::<Transform>(),
                w.write::<CompMoving>(),
            ));

        let mut done = false;

        for (player, transform) in (&players, &transforms).iter() {
            let pos = transform.get_pos();
            if pos.x.abs() > 10.0 || pos.y.abs() > 10.0 || (self.time > 30.0 && player.get_player() == Player::One) {
                self.feeder_front_channel.send_to(ScoreToFeeder::Lose(player.get_player()));
                self.time = 0.0;
                done = true;
                break;
            }
        }

        if done {
            for (player, mut transform, mut moving) in (&players, &mut transforms, &mut movings).iter() {
                transform.set_pos(match player.get_player() {
                    Player::One => PLAYER_ONE_START,
                    Player::Two => PLAYER_TWO_START,
                });
                *moving.get_mut_velocity() = STARTING_VELOCITY;
            }
        }
    }
}
