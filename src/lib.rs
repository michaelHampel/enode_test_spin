use spin_sdk::http::{IntoResponse, Method, Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_hello_spin_rust(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let o_auth_url = std::env::var("OAUTH_URL").unwrap();
    println!("Env OAuth_Urln loaded: {}", o_auth_url);


    let mut router = Router::new();
    router.get_async("enox/flow/enode/httpbin", api::httpbin);
    router.get("enox/flow/enode/test", api::test);
    router.get_async("/*", api::echo_wildcard);

    router.handle(req)
    
}

mod api {
    use spin_sdk::http::Params;

    use super::*;

    pub fn test(_req:Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
        
        Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())

    }

    pub async fn httpbin(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
        let out_req = Request::builder()
        .method(Method::Get)
        .uri("http://httpbin.org/get")
        .build();

        let resp: Response =  spin_sdk::http::send(out_req).await?;

        println!("Http Get Status: {}", resp.status());
        resp.headers()
            .for_each(|x| println!("Header: {:#?}", x));

        let body_str = String::from_utf8(resp.body().to_vec());
        println!("Response body from httpbin: \n{:#?}", body_str);

        Ok(Response::new(200, body_str.unwrap()))
    }

    pub async fn echo_wildcard(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
        let capture = params.wildcard().unwrap_or_default();
        Ok(Response::new(200, capture.to_string()))
    }


}
