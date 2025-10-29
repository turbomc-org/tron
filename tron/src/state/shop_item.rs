use crate::models::shop_item::ShopItem;
use crate::state::State;
use anyhow::Result;
use tonic::Status;
use tracing::error;

impl State {
    pub async fn get_shop_item(&self, id: &u64) -> Result<ShopItem, Status> {
        if let Some(item_ref) = self.shop_items.get(id) {
            Ok(item_ref.clone())
        } else {
            error!("Shop item requested by player not found");
            Err(Status::not_found("Shop item requested not found"))
        }
    }

    pub async fn insert_shop_item(&self, shop_item: ShopItem) -> Result<()> {
        self.shop_items.insert(shop_item.id, shop_item.clone());
        self.shop_item_indexes.insert(
            (
                shop_item.type_id,
                shop_item.enchantments.iter().map(|v| v.clone()).collect(),
            ),
            shop_item.id,
        );
        Ok(())
    }

    pub async fn remove_shop_item(&self, shop_item: &ShopItem) -> Result<()> {
        self.shop_items.remove(&shop_item.id);
        self.shop_item_indexes.remove(&(
            shop_item.type_id.clone(),
            shop_item.enchantments.iter().map(|v| v.clone()).collect(),
        ));
        Ok(())
    }

    pub async fn get_by_typeid_and_enchantments(
        &self,
        type_id: &String,
        enchantments: &[String],
    ) -> Result<ShopItem, Status> {
        if let Some(id) = self
            .shop_item_indexes
            .get(&(type_id.clone(), enchantments.to_vec()))
            .map(|e| e.clone())
        {
            if let Some(item_ref) = self.shop_items.get(&id) {
                Ok(item_ref.clone())
            } else {
                error!("Shop item requested by player not found");
                Err(Status::not_found("Shop item requested not found"))
            }
        } else {
            error!("Shop item requested by player not found");
            Err(Status::not_found("Shop item requested not found"))
        }
    }
}
