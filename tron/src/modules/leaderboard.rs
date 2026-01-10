use crate::models::leaderboards::Leaderboard;
use crate::models::leaderboards::ScoreKey;
use anyhow::Result;
use std::cmp::Reverse;

impl Leaderboard {
    pub async fn update_score(&self, player_id: u64, new_score: u64) {
        let mut order = self.order.write().await;
        if let Some(old_key) = self.index.remove(&player_id) {
            order.remove(&old_key.1);
        }
        let key = ScoreKey(Reverse(new_score), player_id);
        self.index.insert(player_id, key.clone());
        order.insert(key, new_score);
    }

    pub async fn get_rank(&self, player_id: &u64) -> Option<usize> {
        let order = self.order.read().await;
        let key = self.index.get(player_id)?.clone();
        Some(order.rank(&key) + 1)
    }

    pub async fn get(&self, limit: usize) -> Vec<(u64, u64)> {
        self.order
            .read()
            .await
            .iter()
            .take(limit)
            .map(|(ScoreKey(_, id), score)| (*id, *score))
            .collect()
    }

    pub async fn get_score(&self, player_id: u64) -> Result<Option<u64>> {
        let key_ref = match self.index.get(&player_id) {
            Some(k) => k,
            None => return Ok(None),
        };

        let order = self.order.read().await;
        Ok(order.get(&key_ref).cloned())
    }
}
