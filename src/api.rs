use spin_contrib_http::request::Contrib;
use spin_sdk::variables;
use spin_sdk::http::{IntoResponse, Request, Response, Router};
use spin_contrib_http::cors::{
    CorsConfig, CorsResponseBuilder, CorsRouter, ALL_HEADERS, ALL_METHODS, NO_ORIGINS
};
use std::fmt::Display;

use crate::enode_handlers;
use crate::app_handlers;
use crate::test_api;

pub(crate) fn load_cors_config() -> CorsConfig {
    CorsConfig::new(
        variables::get("cors_allowed_origins").unwrap_or(NO_ORIGINS.into()),
        variables::get("cors_allowed_methods").unwrap_or(ALL_METHODS.into()),
        variables::get("cors_allowed_headers").unwrap_or(ALL_HEADERS.into()),
        variables::get("cors_allow_credentials")
            .unwrap_or("true".to_string())
            .parse()
            .unwrap_or(true),
        variables::get("cors_max_age")
            .ok()
            .and_then(|v| v.parse::<u32>().ok()),
    )
}


pub(crate) struct Api {
    router: Router,
}

impl Api {
    pub(crate) fn handle(&self, req: Request) -> anyhow::Result<impl IntoResponse> {
        let method = &req.method().clone();
        let origin = req.get_header_value_as_string("origin");
        Ok(self.router
            .handle(req)
            .into_builder()
            .build_with_cors(method, origin,  &load_cors_config()))
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
        let cors_cfg = load_cors_config();
        let mut router = Router::default();
        router.register_options_handler(&cors_cfg);
        router.get("enox/flow/enode/health", app_handlers::health);
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