use crate::cache::Cache;
use crate::models::player::Player;
use crate::models::team::Team;
use anyhow::Result;
use tonic::Status;
use tracing::error;

impl Cache {
    pub async fn get_team(&self, id: u64) -> Result<Team, Status> {
        match self.teams.get(&id).map(|entry| entry.clone()) {
            Some(team) => Ok(team),
            None => Err(Status::not_found(format!("Team not found"))),
        }
    }

    pub async fn insert_team(&self, team: Team) -> Result<()> {
        self.teams.insert(team.id, team);
        Ok(())
    }

    pub async fn check_team_request(
        &self,
        player: &Player,
        team_name: &str,
    ) -> Result<u64, Status> {
        for (team_id, _now) in player.incoming_team_requests.iter() {
            let team = self.get_team(*team_id).await?;
            if team.name == team_name.to_string() {
                return Ok(*team_id);
            }
        }
        error!(
            "Player {} has no team request from team {}",
            player.username, team_name
        );
        Err(Status::not_found(format!(
            "Player {} has no team request from team {}",
            player.username, team_name
        )))
    }

    pub async fn get_team_by_name(&self, team_name: &str) -> Result<Team, Status> {
        for entry in self.teams.iter() {
            let team = self.get_team(*entry.key()).await?;
            if team.name == team_name {
                return Ok(team);
            }
        }

        Err(Status::not_found(format!("Team {} not found", team_name)))
    }

    pub async fn accept_team_request(
        &self,
        player: &Player,
        team_id: u64,
        now: u64,
    ) -> Result<(), Status> {
        let mut team = self.get_team(team_id).await?;
        let mut player = player.clone();
        team.members.insert(player.id, now);
        self.teams.insert(team.id, team);
        player.incoming_team_requests.remove(&team_id);
        self.active_players.insert(player.username.clone(), player);

        Ok(())
    }
}
