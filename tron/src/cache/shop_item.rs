use crate::cache::Cache;
use crate::models::shop_item::ShopItem;
use anyhow::Result;
use tonic::Status;
use tracing::error;

impl Cache {
    pub async fn get_shop_item(&self, id: &u64) -> Result<ShopItem, Status> {
        if let Some(item_ref) = self.shop_items.get(id) {
            Ok(item_ref.clone())
        } else {
            error!("Shop item requested by player not found");
            Err(Status::not_found("Shop item requested not found"))
        }
    }

    pub async fn insert_shop_item(&self, shop_item: ShopItem) -> Result<()> {
        self.shop_items.insert(shop_item.id, shop_item);
        Ok(())
    }
}
