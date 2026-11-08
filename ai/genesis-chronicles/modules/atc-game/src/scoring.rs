// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Score tracking and leaderboards
use std::collections::BTreeMap;
pub struct Leaderboard { scores: BTreeMap<i64, Vec<String>> }
impl Leaderboard {
    pub fn new() -> Self { Self { scores: BTreeMap::new() } }
    pub fn add_score(&mut self, player: &str, score: i64) {
        self.scores.entry(score).or_default().push(player.into());
    }
    pub fn top_n(&self, n: usize) -> Vec<(String, i64)> {
        self.scores.iter().rev().flat_map(|(s, players)| {
            players.iter().map(move |p| (p.clone(), *s))
        }).take(n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_leaderboard() {
        let mut lb = Leaderboard::new();
        lb.add_score("alice", 100);
        lb.add_score("bob", 200);
        let top = lb.top_n(2);
        assert_eq!(top[0], ("bob".into(), 200));
    }
}
