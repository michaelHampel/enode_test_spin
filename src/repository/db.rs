
use anyhow::Context;
use spin_sdk::sqlite::{Connection, Value};

use crate::models::{DbSuccess, EnodeClientToken, User, UserRegistration};

const QUERY_ENODE_CLIENT_TOKEN: &str = "SELECT TOKEN, LIFETIME FROM enodeTokens WHERE CLIENT = ?";
const UPSERT_ENODE_CLIENT_TOKEN: &str = "INSERT INTO enodeTokens (client, token, lifetime) VALUES(?, ?, ?) ON CONFLICT(client) DO UPDATE SET token=excluded.token, lifetime=excluded.lifetime ";
const COMMAND_CREATE_USER: &str = "INSERT INTO USERS (EMAIL, FIRSTNAME, LASTNAME, PWD) VALUES (?,?,?,?)";
const QUERY_USER: &str = "Select EMAIL, FIRSTNAME, LASTNAME, PWD FROM users WHERE EMAIL = ?";

pub(crate) fn get_client_token(client: String) -> anyhow::Result<Option<EnodeClientToken>> {
    println!("get_client_token for: {}", client);
    let con = Connection::open_default()?;
    let params = [Value::Text(client.clone())];
    let query_result = con.execute(QUERY_ENODE_CLIENT_TOKEN, &params)?;
    let res = match query_result.rows().next() {
        None => None,
        Some(row) => Some(EnodeClientToken {
            client: client,
            token: row.get::<&str>("token").unwrap_or_default().to_string(),
            lifetime: row.get::<i64>("lifetime").unwrap(),
        }),
    };

    Ok(res)
}

pub(crate) fn upsert_client_token(client: String, token:String, lifetime: i64) -> anyhow::Result<EnodeClientToken> {
  println!("upsert token for: {}", client);
  let con = Connection::open_default()?;
  let params = [
    Value::Text(client.clone()), 
    Value::Text(token.clone()), 
    Value::Integer(lifetime.clone().into())
  ];

  con.execute(
    UPSERT_ENODE_CLIENT_TOKEN, 
    params.as_slice())
    .and(Ok(EnodeClientToken { client, token, lifetime}))
    .with_context(|| "Error while upserting client token in DB!!")
}

pub(crate) fn create_user(data: UserRegistration) -> anyhow::Result<User> {
  println!("Inserting user in db: {:#?}", data);
  let con = Connection::open_default()?;

  let params = [
    Value::Text(data.email.clone()),
    Value::Text(data.first_name.clone()),
    Value::Text(data.last_name.clone()),
    Value::Text(data.pwd.clone()),
  ];

  con.execute(COMMAND_CREATE_USER, &params)
  .and(Ok(User{email: data.email, first_name: data.first_name, last_name: data.last_name}))
  .with_context(|| "Error while inserting user in DB!!")

}

pub(crate) fn get_user(email: String) -> anyhow::Result<Option<User>> {
  println!("get user: {}", email);
  let con = Connection::open_default()?;
  let params = [Value::Text(email.clone())];
  let query_result = con.execute(QUERY_USER, &params)?;
  let res = match query_result.rows().next() {
      None => None,
      Some(row) => Some(User {
          email: email,
          first_name: row.get::<&str>("firstname").unwrap_or_default().to_string(),
          last_name: row.get::<&str>("lastname").unwrap_or_default().to_string(),
      }),
  };

  Ok(res)
}