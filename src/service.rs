use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use moka::future::Cache;

use crate::handlers::CartItem;
#[derive(Clone)]
pub struct Service {
    carts: Cache<String, Arc<Mutex<Vec<CartItem>>>>,
}

impl Service {
    pub fn new() -> Self {
        let carts = Cache::builder()
            .time_to_idle(Duration::from_hours(12))
            .max_capacity(1000)
            .build();
        Self { carts }
    }

    pub async fn remove(&self, cart_id: String, product_id: String) {
        let products = match self.carts.get(&cart_id).await {
            Some(r) => r,
            None => return,
        };

        products
            .lock()
            .unwrap()
            .retain(|p| *p.product != product_id);
        self.carts.insert(cart_id, products).await;
    }

    pub async fn add(&self, cart_id: String, item: CartItem) {
        let products = self
            .carts
            .get(&cart_id)
            .await
            .unwrap_or(Arc::new(Mutex::new(Vec::new())));
        products
            .lock()
            .unwrap()
            .retain(|p| p.product != item.product);
        products.lock().unwrap().push(item);
        self.carts.insert(cart_id, products).await;
    }

    pub async fn get(&self, cart_id: String) -> Vec<CartItem> {
        match self.carts.get(&cart_id).await {
            Some(r) => r.lock().unwrap().clone(),
            None => Vec::new(),
        }
    }
}
