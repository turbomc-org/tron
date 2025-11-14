use dashmap::DashSet;

#[derive(Debug)]
pub struct Permissions {
    pub admins: DashSet<u64>,
    pub moderators: DashSet<u64>,
}

impl Permissions {
    pub fn new() -> Self {
        Self {
            admins: DashSet::new(),
            moderators: DashSet::new(),
        }
    }
}

impl Permissions {
    pub fn add_admin(&self, player_id: u64) {
        self.admins.insert(player_id);
    }

    pub fn add_moderator(&self, player_id: u64) {
        self.moderators.insert(player_id);
    }

    pub fn remove_admin(&self, player_id: &u64) {
        self.admins.remove(player_id);
    }

    pub fn remove_moderator(&self, player_id: &u64) {
        self.moderators.remove(player_id);
    }
}
