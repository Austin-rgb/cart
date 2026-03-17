use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use moka::future::Cache;
#[derive(Clone)]
pub struct Service {
    carts: Cache<String, Arc<Mutex<Vec<(String, u32)>>>>,
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

        products.lock().unwrap().retain(|p| *p.0 != product_id);
        self.carts.insert(cart_id, products).await;
    }

    pub async fn add(&self, cart_id: String, product_id: String, quantity: u32) {
        let products = self
            .carts
            .get(&cart_id)
            .await
            .unwrap_or(Arc::new(Mutex::new(Vec::new())));
        products.lock().unwrap().retain(|p| p.0 != product_id);
        products.lock().unwrap().push((product_id, quantity));
        self.carts.insert(cart_id, products).await;
    }

    pub async fn get(&self, cart_id: String) -> Vec<(String, u32)> {
        match self.carts.get(&cart_id).await {
            Some(r) => r.lock().unwrap().clone(),
            None => Vec::new(),
        }
    }
}
