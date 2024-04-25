use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;
use api::Api;

mod enode_handlers;
mod app_handlers;
mod models;
mod repository;
mod test_api;
mod api;
mod cors;


/// A simple Spin HTTP component.
#[http_component]
fn handle_enode_test_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let api = Api::default();
    api.handle(req)   
    
}





/*


#[derive(Debug, Default)]
struct TokenRepo {
    user_token: Option<EnodeTokenResponse>
}

impl TokenRepo {
    pub fn init() -> Self {
        Self {
            user_token: None
        }
    }
    
    fn upd(self) {
        TOKEN_REPO.with(|t| *t.write().unwrap() = Arc::new(self))
    }

    pub fn get() -> Arc<TokenRepo> {
        TOKEN_REPO.with(|t| t.read().unwrap().clone())
    }

    pub fn update(&mut self, token: EnodeTokenResponse) {
        println!("[update] old token: {:#?}", self.user_token);
        self.user_token = Some(token);
        println!("[update] updated token: {:#?}", self.user_token);

    }

    pub async fn get_token(&mut self) -> anyhow::Result<EnodeTokenResponse> {
        match &self.user_token {
            Some(token) => {
                println!("Found token!!");
                return Ok(token.clone())
            },
            None => {
                println!("No access_token - query new one!!");
                let enode_token = api::query_token().await?;
                self.update(enode_token.clone());
                return Ok(enode_token)   
            },   
        }
    }

    
}*/

// static TOKEN_REPO: Mutex<TokenRepo> = Mutex::new(TokenRepo { user_token: None });

/*let token_str = match TokenRepo::get().user_token.clone() {
            Some(token) => {
                println!("Found token!!");
                token.header_str()
            },
            None => {
                println!("No access_token - query new one!!");
                let enode_token = api::query_token().await?;
                TokenRepo::upd(TokenRepo { user_token: Some(enode_token.clone())});
                enode_token.header_str() 
            },       
        };*/

// let token_str: String = TOKEN_REPO.lock().unwrap().get_token().await?.header_str();