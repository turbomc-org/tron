use crate::BridgeService;
use crate::models::team::Team;
use tonic::{Request, Response, Status};
use tracing::{debug, error};
use tron_protos::{CreateTeamRequest, CreateTeamResponse};

impl BridgeService {
    pub async fn handle_create_team(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let team_name = inner_request.team;
        let color = inner_request.color;
        let open = inner_request.open;

        debug!("Create team request from player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;

        if player.team.is_some() {
            error!("Player {} is already in a team", username);
            return Err(Status::already_exists(
                "You are already in a team first leave team in order to create new",
            ));
        }

        if self.state().indexes.team.contains_key(&team_name) {
            error!("Team already exists with this name");
            return Err(Status::already_exists(format!(
                "Team already exists with name {}",
                team_name,
            )));
        }

        let team = Team::new(team_name.clone(), player.id.clone(), open, color);

        team.insert(&self.collections().teams, &self.state())
            .await
            .map_err(|e| {
                error!("Failed to create team: {}", e);
                Status::internal("Failed to create team")
            })?;

        player
            .set_team(team.id, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to set team for player: {}", err);
                Status::internal("Failed to set team for player")
            })?;

        if let Err(err) = self.send_message(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>✅ SQUAD INITIALIZED</bold></gradient>\n\
             <gray>You have successfully formed the <white><bold>{}</bold></white> squad.</gray>\n\
             <dark_gray>»</dark_gray> <gray>Use <white>/team invite <user></white> to expand your roster.</gray>",
            team_name
          ),
        ).await {
            error!("Failed to send message to player: {}", err);
        };

        debug!("Create team request from player {} completed", username);
        debug!("Created team {} for player {}", team_name, username);

        Ok(Response::new(CreateTeamResponse { success: true }))
    }
}
