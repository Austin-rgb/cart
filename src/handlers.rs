use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::service::Service;
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::Cookie,
    web::{Data, Path, Query},
};

#[derive(Deserialize)]
pub struct CartItem {
    product: String,
    qty: u32,
}

pub async fn add(
    service: Data<Service>,
    req: HttpRequest,
    data: Query<CartItem>,
) -> impl Responder {
    let item = data.into_inner();
    if let Some(cart_id) = req.cookie("cart_id") {
        service
            .add(cart_id.value().to_owned(), item.product, item.qty)
            .await;
        HttpResponse::Ok().finish()
    } else {
        let cart_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string();
        let cookie = Cookie::build("cart_id", cart_id).finish();
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
