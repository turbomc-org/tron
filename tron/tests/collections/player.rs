// #[tokio::test]
// async fn test_insert_player_inserts_in_cache_and_db() {
//     use dashmap::DashMap;
//     use std::sync::Arc;
//     use tron::collections::player::{MockPlayerCollection, PlayerCollection};
//     use tron::models::player::{Edition, Player};

//     let mut mock = MockPlayerCollection::new();
//     let player = Player::new("harihar".to_string(), Edition::Java);

//     let player_clone = player.clone();
//     mock.expect_insert_one().times(1).returning(move |p| {
//         assert_eq!(p.username, player_clone.username);
//         Ok(())
//     });

//     let col: Arc<dyn PlayerCollection> = Arc::new(mock);
//     let cache = Arc::new(DashMap::new());

//     player.insert(&col, &state).await.unwrap();

//     assert!(cache.contains_key("harihar"));

//     tokio::time::sleep(std::time::Duration::from_millis(50)).await;
// }
