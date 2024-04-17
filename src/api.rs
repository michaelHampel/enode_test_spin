use spin_sdk::http::{Request, IntoResponse, Router};


use std::fmt::Display;

use crate::enode_handlers;
use crate::test_api;

pub(crate) struct Api {
    router: Router,
}

impl Api {
    pub(crate) fn handle(&self, req: Request) -> anyhow::Result<impl IntoResponse> {
        Ok(self.router.handle(req))
    }
}

impl Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.router)
    }
}

impl Default for Api {
    fn default() -> Self {
        println!("Called API default...");
        let mut router = Router::new();
        router.get_async("enox/flow/enode/users/linksandbox", enode_handlers::link_sandbox_bev);
        router.post_async("enox/flow/enode/users/link", enode_handlers::link_user_resource);
        router.get_async("enox/flow/enode/users", enode_handlers::get_users);
        router.get_async("enox/flow/enode/users/:userId", enode_handlers::get_user);
        router.get_async("enox/flow/enode/users/:userId/vehicles", enode_handlers::get_user_vehicles);
        router.get_async("enox/flow/enode/users/unlink/:userId", enode_handlers::unlink_user);
        router.get_async("enox/flow/enode/vehicles", enode_handlers::get_vehicles);
        router.get_async("enox/flow/enode/vehicles/:vehicleId", enode_handlers::get_vehicle);
        router.post_async("enox/flow/enode/vehicles/:vehicleId/charging", enode_handlers::charge_vehicle);
        router.get_async("enox/flow/enode/vehicles/action/:actionId", enode_handlers::get_vehicle_action);
        router.get_async("enox/flow/enode/httpbin", test_api::httpbin);
        router.get("enox/flow/enode/test", test_api::test);
        router.get("enox/flow/enode/testdb", test_api::test_db);
        router.get("/*", test_api::echo_wildcard);
        
        Self { router: router }
    }
}