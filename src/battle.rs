use crate::single_battle::SingleBattle;

pub(crate) enum Battle {
    Single(SingleBattle),
    // Double(DoubleBattle),
}

impl Battle {
    pub(crate) fn run(&mut self) {
        todo!()
    }
}