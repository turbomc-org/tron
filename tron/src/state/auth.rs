use crate::state::State;

impl State {
    pub fn get_alias(&self, username: &str) -> Option<String> {
        self.aliases.get(username).map(|alias| alias.clone())
    }
}
