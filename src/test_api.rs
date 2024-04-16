use serde::Serialize;
use spin_sdk::{http::{IntoResponse, Method, Params, Request, Response}, sqlite::{Connection, Value}};



pub fn test(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
        
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

pub fn echo_wildcard(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let capture = params.wildcard().unwrap_or_default();
    Ok(Response::new(200, capture.to_string()))
}

pub fn test_db(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let connection = Connection::open_default()?;
    
    let execute_params = [
        Value::Text("Try out Spin SQLite".to_owned()),
        Value::Text("Friday".to_owned()),
    ];
    connection.execute(
        "INSERT INTO todos (description, due) VALUES (?, ?)",
        execute_params.as_slice(),
    )?;
    
    let rowset = connection.execute(
        "SELECT id, description, due FROM todos",
        &[]
    )?;
    
    let todos: Vec<_> = rowset.rows().map(|row|
        ToDo {
            id: row.get::<u32>("id").unwrap(),
            description: row.get::<&str>("description").unwrap().to_owned(),
            due: row.get::<&str>("due").unwrap().to_owned(),
        }
    ).collect();
    
    let body = serde_json::to_vec(&todos)?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(body)
        .build())
}
    
// Helper for returning the query results as JSON
#[derive(Serialize)]
struct ToDo {
    id: u32,
    description: String,
    due: String,
}