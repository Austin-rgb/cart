use actix_web::web::{Data, ServiceConfig, get, scope};

use crate::{
    handlers::{add, remove},
    service::Service,
};

pub struct Module {
    pub service: Service,
}

impl Module {
    pub fn new() -> Self {
        let service = Service::new();
        Self { service }
    }
    pub fn config(&self, mut cfg: ServiceConfig, namespace: &str) {
        cfg.service(
            scope(namespace)
                .app_data(Data::new(self.service.clone()))
                .route("/add", get().to(add))
                .route("/remove", get().to(remove)),
        );
    }
}
