use dependencies::cgmath::{Vector3};
use utils::{Player, Coord};

#[derive(Debug)]
pub enum FeederToAi {
    PlayerPosition(Player, Vector3<Coord>),
}

#[derive(Debug)]
pub enum FeederFromAi {

}
