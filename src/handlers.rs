use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::service::Service;
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::Cookie,
    web::{Data, Path, Query},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CartItem {
    pub product: String,
    pub qty: u32,
}

pub async fn add(
    service: Data<Service>,
    req: HttpRequest,
    data: Query<CartItem>,
) -> impl Responder {
    let item = data.into_inner();
    if let Some(cart_id) = req.cookie("cart_id") {
        let cart_id = cart_id.value();
        service.add(cart_id.to_owned(), item).await;
        HttpResponse::Ok().finish()
    } else {
        let cart_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string();
        service.add(cart_id.clone(), item).await;
        let cookie = Cookie::build("cart_id", cart_id)
            // Set path to root '/'
            // This is very important as actix-web defaults to setting it to current path making it unusable from other paths
            // You can think setting it to something else, but that will change how it's supposed to be accessed given that the root path might be configured differently in the main app
            .path("/")
            .finish();
        HttpResponse::Ok().cookie(cookie).finish()
    }
}

pub async fn remove(
    service: Data<Service>,
    req: HttpRequest,
    product: Path<String>,
) -> impl Responder {
    let product = product.into_inner();
    if let Some(cart_id) = req.cookie("cart_id") {
        service.remove(cart_id.value().to_owned(), product).await;
    }
    HttpResponse::Ok().finish()
}

pub async fn get(service: Data<Service>, req: HttpRequest) -> impl Responder {
    let mut cart = Vec::new();
    if let Some(cart_id) = req.cookie("cart_id") {
        cart = service.get(cart_id.value().to_owned()).await;
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .body(to_string(&cart).unwrap_or("".to_string()))
}
