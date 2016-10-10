extern crate dependencies;
#[macro_use]
extern crate log;

pub use dependencies::rustc_serialize;

pub mod fps_counter;

pub use fps_counter::FpsCounter;

pub type Delta = f64;
pub type Coord = f32;
pub type CoordI = i64;
pub type GfxCoord = f32;

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub struct WindowId(pub u32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, RustcEncodable, RustcDecodable)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn map_player(player: Player) -> i32 {
        match player {
            Player::One => 0,
            Player::Two => 1,
        }
    }

    pub fn map_number(number: i32) -> Option<Player> {
        match number {
            0 => Some(Player::One),
            1 => Some(Player::Two),
            _ => None,
        }
    }
}
