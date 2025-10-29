use crate::models::player::Player;
use crate::models::team::Team;
use crate::state::State;
use anyhow::Result;
use tonic::Status;
use tracing::error;

impl State {
    pub async fn get_team(&self, id: u64) -> Result<Option<Team>> {
        Ok(self.teams.get(&id).map(|entry| entry.value().clone()))
    }

    pub async fn get_team_by_name(&self, name: String) -> Result<Option<Team>> {
        let id = match self.team_indexes.get(&name) {
            Some(entry) => *entry.value(),
            None => return Ok(None),
        };

        self.get_team(id).await
    }

    pub async fn get_team_id(&self, name: String) -> Result<Option<u64>> {
        let id = match self.team_indexes.get(&name) {
            Some(entry) => *entry.value(),
            None => return Ok(None),
        };

        Ok(Some(id))
    }

    pub async fn get_team_with_handling(&self, id: u64) -> Result<Team, Status> {
        match self.teams.get(&id).map(|entry| entry.clone()) {
            Some(team) => Ok(team),
            None => Err(Status::not_found(format!(
                "You are not a member of any team"
            ))),
        }
    }

    pub async fn insert_team(&self, team: Team) -> Result<()> {
        self.teams.insert(team.id, team.clone());
        self.team_indexes.insert(team.name, team.id);
        Ok(())
    }

    pub async fn check_team_request(
        &self,
        player: &Player,
        team_name: &str,
    ) -> Result<u64, Status> {
        for (team_id, _now) in player.incoming_team_requests.iter() {
            let team = self.get_team_with_handling(*team_id).await?;
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
}
