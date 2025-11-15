use crate::models::redeem::Redeem;
use crate::state::State;

impl State {
    pub fn insert_redeem(&self, redeem: Redeem) {
        self.redeems.insert(redeem.id.clone(), redeem.clone());
        self.indexes.redeem.insert(redeem.code, redeem.id);
    }

    pub fn remove_redeem(&self, id: &u64, code: &String) {
        self.redeems.remove(id);
        self.indexes.redeem.remove(code);
    }
}
