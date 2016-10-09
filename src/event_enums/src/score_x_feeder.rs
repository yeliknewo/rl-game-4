use utils::{Player};

#[derive(Debug)]
pub enum ScoreToFeeder {
    Lose(Player),
    LoseBoth,
}

#[derive(Debug)]
pub enum ScoreFromFeeder {

}
