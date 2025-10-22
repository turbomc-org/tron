use crate::cache::Cache;
use crate::models::shop_item::ShopItem;
use anyhow::Result;

impl Cache {
    pub async fn get_shop_item(&self, id: &u64) -> Result<Option<ShopItem>> {
        Ok(self.shop_items.get(id).map(|entry| entry.clone()))
    }

    pub async fn insert_shop_item(&self, shop_item: ShopItem) -> Result<()> {
        self.shop_items.insert(shop_item.id, shop_item);
        Ok(())
    }
}
