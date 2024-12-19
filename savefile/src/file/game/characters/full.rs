use crate::bin_bool::BinBool;

use super::{Character, CharacterIndex};

/// Full list of characters.
///
/// ORDER MATTERS. That's how they are coded in game.
///
/// Also see [`StoryCharacterSheet`].
#[binrw::binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[brw(little)]
pub struct FullCharacterSheet {
    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sora: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub alte: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub tsih: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub mira: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sham: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nath: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub star_breaker: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub suguri: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub saki: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub iru: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub nanako: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub kae: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub kyoko: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub hime: bool,

    #[br(map = From::<BinBool>::from)]
    #[bw(map = BinBool::from)]
    pub sumika: bool,
}

#[derive(Debug, Clone)]
pub struct FullCharacterValueGrid<T> {
    grid: [T; 15],
    index: usize,
}

impl FullCharacterSheet {
    pub const FULLY_UNLOCKED: Self = Self {
        sora: true,
        alte: true,
        tsih: true,
        mira: true,
        sham: true,
        nath: true,
        star_breaker: true,
        suguri: true,
        saki: true,
        iru: true,
        nanako: true,
        kae: true,
        kyoko: true,
        hime: true,
        sumika: true,
    };
}

impl<T> FullCharacterValueGrid<T> {
    pub fn switch_next(&mut self) {
        self.index = self.index.saturating_add(1).clamp(0, self.grid.len() - 1);
    }

    pub fn switch_previous(&mut self) {
        self.index = self.index.saturating_sub(1).clamp(0, self.grid.len() - 1);
    }

    pub fn current_index(&self) -> usize {
        self.index
    }

    pub fn get_current(&self) -> &T {
        self.grid
            .get(self.index)
            .expect("Invariant Broken: bad index value")
    }

    pub fn get_current_mut(&mut self) -> &mut T {
        self.grid
            .get_mut(self.index)
            .expect("Invariant Broken: bad index value")
    }

    pub fn characters(&self) -> impl Iterator<Item = (Character, &T)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .map(|(index, item)| (CharacterIndex::from(index).into(), item))
    }
}

impl FullCharacterValueGrid<bool> {
    pub fn toggle_current(&mut self) {
        let current = self.get_current_mut();
        *current = !*current;
    }
}

impl From<FullCharacterSheet> for FullCharacterValueGrid<bool> {
    fn from(
        FullCharacterSheet {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        }: FullCharacterSheet,
    ) -> Self {
        let grid = [
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        ];
        Self { grid, index: 0 }
    }
}

impl From<FullCharacterValueGrid<bool>> for FullCharacterSheet {
    fn from(
        FullCharacterValueGrid {
            grid:
                [sora, alte, tsih, mira, sham, nath, star_breaker, suguri, saki, iru, nanako, kae, kyoko, hime, sumika],
            index: _,
        }: FullCharacterValueGrid<bool>,
    ) -> Self {
        Self {
            sora,
            alte,
            tsih,
            mira,
            sham,
            nath,
            star_breaker,
            suguri,
            saki,
            iru,
            nanako,
            kae,
            kyoko,
            hime,
            sumika,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file::game::characters::full::{FullCharacterSheet, FullCharacterValueGrid};

    #[rstest::rstest]
    fn grid_properly_switches_indexes() {
        let mut grid = FullCharacterValueGrid::from(FullCharacterSheet::default());
        assert_eq!(grid.current_index(), 0);

        grid.switch_previous();
        assert_eq!(grid.current_index(), 0);

        grid.switch_next();
        assert_eq!(grid.current_index(), 1);

        for _ in 0..100 {
            grid.switch_next();
        }
        assert_eq!(grid.current_index(), 14);
    }

    #[rstest::rstest]
    fn negates_properly() {
        let mut grid = FullCharacterValueGrid::from(FullCharacterSheet::default());

        assert_eq!(*grid.get_current(), false);

        grid.toggle_current();
        assert_eq!(*grid.get_current(), true);

        grid.toggle_current();
        assert_eq!(*grid.get_current(), false);
    }
}
