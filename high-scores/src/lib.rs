#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.iter().map(|s| s.to_owned()).last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().map(|s| s.to_owned()).max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three: Vec<u32> = self.0.clone();
        top_three.sort_by(|a, b| b.cmp(a));
        top_three.iter().take(3).map(|s| s.to_owned()).collect()
    }
}
