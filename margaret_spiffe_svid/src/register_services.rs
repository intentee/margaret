use trzcina::Service;
use trzcina::ServiceManager;

use crate::boxed_service::BoxedService;

pub fn register_services(manager: &mut ServiceManager, services: Vec<Box<dyn Service>>) {
    for service in services {
        manager.register_service(BoxedService::new(service));
    }
}
