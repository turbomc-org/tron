use crate::models::leaderboards::Leaderboard;
use crate::models::leaderboards::ScoreKey;
use anyhow::Result;
use std::borrow::Borrow;
use std::cmp::Reverse;

impl Leaderboard {
    pub async fn update_score(&self, player_id: u64, new_score: u64) {
        if let Some(old_key) = self.index.remove(&player_id) {
            self.order.write().await.remove(&old_key.1);
        }
        let key = ScoreKey(Reverse(new_score), player_id);
        self.index.insert(player_id, key.clone());
        self.order.write().await.insert(key, new_score);
    }

    pub async fn get_rank(&self, player_id: impl Borrow<u64>) -> Option<usize> {
        let player_id: &u64 = player_id.borrow();
        let key = self.index.get(player_id)?.clone();
        let order = self.order.read().await;
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
        let key = self.index.get(&player_id);

        if key.is_none() {
            return Ok(None);
        }

        let order = self.order.read().await;
        Ok(order.get(&key.unwrap()).cloned())
    }
}
