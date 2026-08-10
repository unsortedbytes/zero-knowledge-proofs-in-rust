// #[warn(unused_imports)]
use tonic::{transport::Server, Code, Request, Response, Status};
use zkp_auth::{auth_server::{Auth, AuthServer}, RegisterRequest, RegisterResponse,} ;

use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse};


pub mod zkp_auth{
    include!("./zkp_auth.rs");
}

#[derive(Debug, Default)]
struct AuthImpl{}

// #[derive(Debug, Default)]{
//     pub user_name : String,
//     pub y1: 
// }

#[tonic::async_trait]
impl Auth for AuthImpl{
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        todo!()
    }

    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status>{
        todo!()
    }

    async fn verify_authentication(&self, request:Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status>{
        todo!()
    }
}

// use  zkp_auth::auth_server;
#[tokio::main]
async fn main(){

    let addr = "127.0.0.1:50051".to_string();
    println!("Running the Server in {}", addr);

    let auth_impl = AuthImpl::default();
    
    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("could not convert addresss")).await.unwrap();
}