#[derive(Debug, Default)]
pub struct MatchResults {
    pub wins_0: usize,
    pub scores_0: u32,
    pub wins_1: usize,
    pub scores_1: u32,
    pub draws: usize,
}

impl MatchResults {
    pub fn games(&self) -> usize {
        self.wins_0 + self.wins_1 + self.draws
    }
}
