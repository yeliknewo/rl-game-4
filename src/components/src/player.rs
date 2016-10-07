use dependencies::specs::{Component, VecStorage};

pub struct CompPlayer {

}

impl Component for CompPlayer {
    type Storage = VecStorage<CompPlayer>;
}
