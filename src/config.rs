use actix_web::web::{Data, ServiceConfig, get, scope};

use crate::{handlers, service::Service};

#[derive(Clone)]
pub struct Module {
    pub service: Service,
}

impl Module {
    pub fn new() -> Self {
        let service = Service::new();
        Self { service }
    }
    pub fn config(&self, cfg: &mut ServiceConfig, namespace: &str) {
        cfg.service(
            scope(namespace)
                .app_data(Data::new(self.service.clone()))
                .route("/add", get().to(handlers::add))
                .route("/remove/{id}", get().to(handlers::remove))
                .route("/get", get().to(handlers::get)),
        );
    }
}
