use crate::models::player::Player;
use crate::models::team::Team;
use crate::state::State;
use anyhow::Result;
use tonic::Status;
use tracing::error;

impl State {
    pub fn get_team(&self, id: u64) -> Option<Team> {
        self.teams.get(&id).map(|entry| entry.value().clone())
    }

    pub async fn delete_team(&self, team_id: u64, team_name: &String) -> Result<()> {
        self.teams.remove(&team_id);
        self.indexes.team.remove(team_name);
        Ok(())
    }

    pub fn get_team_by_name(&self, name: String) -> Option<Team> {
        let id = match self.indexes.team.get(&name) {
            Some(entry) => *entry.value(),
            None => return None,
        };

        self.get_team(id)
    }

    pub async fn get_team_id(&self, name: String) -> Result<Option<u64>> {
        let id = match self.indexes.team.get(&name) {
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

        if team.open {
            self.indexes.open_team.insert(team.id);
        }

        self.indexes.team.insert(team.name, team.id);
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

    pub async fn update_team_score(&self, team: Team) -> Result<()> {
        let mut team_overall = 0;

        for member in team.members {
            let member_id = member.0;
            let overall_score = self
                .leaderboards
                .overall
                .get_score(member_id)
                .await?
                .unwrap_or(0);
            team_overall += overall_score;
        }

        self.leaderboards
            .teams
            .update_score(team.id, team_overall)
            .await;

        Ok(())
    }

    pub fn get_member_id(&self, team: &Team, username: &str) -> Option<u64> {
        for member in team.members.iter() {
            if let Some(member_username) = self.get_player_username(member.0) {
                if member_username == username {
                    return Some(member.0.clone());
                }
            }
        }

        None
    }
}
