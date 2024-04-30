/*use std::fmt::Display;

use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, ResponseBuilder},
    variables,
};

/// Constant to all all for a CORS property
pub const ALL: &str = "*";
/// Constant for allowing no origins in CORS
pub const NO_ORIGINS: &str = "NULL";

pub const HEADER_ORIGIN: &str               = "Origin";
pub const HEADER_CONTROL_METHOD: &str       = "Access-Control-Request-Method";
pub const HEADER_ALLOW_ORIGIN: &str         = "Access-Control-Allow-Origin";
pub const HEADER_ALLOW_METHODS: &str        = "Access-Control-Allow-Methods";
pub const HEADER_ALLOW_HEADERS: &str        = "Access-Control-Allow-Headers";
pub const HEADER_ALLOW_CREDENTIALS: &str    = "Access-Control-Allow-Credentials";
pub const HEADER_CONTROL_MAX_AGE: &str      = "Access-Control-Max-Age";



#[derive(Debug, Clone)]
pub struct CorsConfig {
    /// The origins to allow in CORS (separated by commas)
    pub allowed_origins: String,
    /// The HTTP methods to allow in CORS (separated by commas)
    pub allowed_methods: String,
    /// The HTTP headers to allow in CORS (separated by commas)
    pub allowed_headers: String,
    /// Whether or not to allow credentials in CORS
    pub allow_credentials: bool,
    /// The max age to allow in CORS
    pub max_age: Option<u32>,
}

impl Display for CorsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CORS configuration")?;
        writeln!(f, " - allowed origins: {}", self.allowed_origins)?;
        writeln!(f, " - allowed methods: {}", self.allowed_methods)?;
        writeln!(f, " - allowed headers: {}", self.allowed_headers)?;
        writeln!(f, " - allow credentials: {}", self.allow_credentials)?;
        writeln!(f, " - max age: {:?}", self.max_age)?;
        Ok(())
    }
}

impl CorsConfig {
    fn load() -> Self {
        CorsConfig {
            allowed_origins: variables::get("cors_allowed_origins").unwrap_or(ALL.into()),
            allowed_methods: variables::get("cors_allowed_methods").unwrap_or(ALL.into()),
            allowed_headers: variables::get("cors_allowed_headers").unwrap_or(ALL.into()),
            allow_credentials: variables::get("cors_allow_credentials")
                .unwrap_or("true".to_string())
                .parse()
                .unwrap_or(true),
            max_age: variables::get("cors_max_age")
                .ok()
                .and_then(|v| v.parse::<u32>().ok()),
        }
    }
    fn is_allowed(&self, kind: &str, value: &str) -> bool {
        let config_value = match kind {
            "origin" => self.allowed_origins.as_str(),
            "method" => self.allowed_methods.as_str(),
            "header" => self.allowed_headers.as_str(),
            _ => return false,
        };
        if config_value.is_empty() {
            return false;
        }
        if kind == "origin" && config_value == NO_ORIGINS {
            return false;
        }
        if config_value == ALL {
            return true;
        }
        let all_allowed: Vec<&str> = config_value.split(',').collect();
        for allowed in all_allowed {
            if allowed == value {
                return true;
            }
        }
        false
    }
    /// Checks if the provided origin is allowed
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        self.is_allowed("origin", origin)
    }

    /// Checks if the provided HTTP method is allowed
    pub fn is_method_allowed(&self, method: &str) -> bool {
        self.is_allowed("method", method)
    }
}

pub fn handle_preflight(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let cors_config = CorsConfig::load();

    let Some(origin) = req.header(HEADER_ORIGIN) else {
        return Ok(Response::new(204, ()));
    };
    let Some(access_control_req_method) =
        req.header(HEADER_CONTROL_METHOD)
    else {
        return Ok(Response::new(204, ()));
    };

    let Some(origin) = origin.as_str() else {
        return Ok(Response::new(204, ()));
    };
    let Some(access_control_req_method) = access_control_req_method.as_str() else {
        return Ok(Response::new(204, ()));
    };

    if cors_config.is_origin_allowed(origin)
        && cors_config.is_method_allowed(access_control_req_method)
    {
        return Ok(Response::builder().status(204).with_cors().body(()).build());
    }
    Ok(Response::new(204, ()))
}

pub trait WithCors {
    fn with_cors(&mut self) -> &mut ResponseBuilder;
}

impl WithCors for ResponseBuilder {
    fn with_cors(&mut self) -> &mut ResponseBuilder {
        let cfg = CorsConfig::load();
        let mut origin = cfg.allowed_origins.as_str();
        if origin.is_empty() {
            origin = NO_ORIGINS;
        }
        self.header(HEADER_ALLOW_ORIGIN, origin)
            .header(
                HEADER_ALLOW_METHODS,
                cfg.allowed_methods.as_str(),
            )
            .header(
                HEADER_ALLOW_HEADERS,
                cfg.allowed_headers.as_str(),
            )
            .header(
                HEADER_ALLOW_CREDENTIALS,
                format!("{}", cfg.allow_credentials),
            );
        if cfg.max_age.is_some() {
            self.header(
                HEADER_CONTROL_MAX_AGE,
                format!("{}", cfg.max_age.unwrap()),
            );
        }
        self
    }
}*/