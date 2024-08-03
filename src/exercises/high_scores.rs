#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from_iter(scores.iter().map(|x| *x)),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(x) = self.scores.last() {
            return Some(*x);
        }
        None
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None;
        }

        let mut res = Vec::from_iter(self.scores.iter().map(|x| *x));
        res.sort_by(|a, b| b.cmp(a));
        Some(*res.first().unwrap())
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut res = Vec::from_iter(self.scores.iter().map(|x| *x));
        res.sort_by(|a, b| b.cmp(a));
        res.iter().take(3).map(|x| *x).collect()
    }
}
