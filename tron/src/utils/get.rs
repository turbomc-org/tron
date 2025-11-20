use tonic::Status;

use crate::BridgeService;
use crate::models::player::Player;
use crate::models::team::Team;

impl BridgeService {
    pub async fn player(&self, username: &String) -> Result<Player, Status> {
        let player = self
            .state()
            .active_players
            .get(username)
            .map(|entry| entry.clone());
        match player {
            Some(player) => Ok(player),
            None => {
                return self
                    .status(
                        username,
                        Status::not_found(format!("Player {} not found", username)),
                    )
                    .await;
            }
        }
    }

    pub async fn team(&self, username: &String, name: &String) -> Result<Team, Status> {
        let id = match self.state().indexes.team.get(name) {
            Some(id) => id,
            None => {
                return self
                    .status(
                        username,
                        Status::not_found(format!("Team with name {} not found", name)),
                    )
                    .await;
            }
        };

        match self.state().teams.get(&id) {
            Some(team) => Ok(team.value().clone()),
            None => {
                return self
                    .status(username, Status::not_found("Team record not found."))
                    .await;
            }
        }
    }

    pub async fn team_by_id(&self, username: &String, id: &u64) -> Result<Team, Status> {
        match self.state().teams.get(id) {
            Some(team) => Ok(team.value().clone()),
            None => {
                return self
                    .status(username, Status::not_found("Team record not found."))
                    .await;
            }
        }
    }
}
