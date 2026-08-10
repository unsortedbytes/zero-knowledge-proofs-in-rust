// #[warn(unused_imports)]
use tonic::{Code, Request, Response, Status, transport::Server};
use zkp_auth::{auth_server::{Auth, AuthServer}, RegisterRequest, RegisterResponse,} ;

use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse};
use num_bigint::BigUint;
use std::{collections::HashMap, fmt::format, hash::Hash, sync::Mutex};

use zero_knowledge_proofs::ZKP;

pub mod zkp_auth{
    include!("./zkp_auth.rs");
}

#[derive(Debug, Default)]
pub struct AuthImpl{
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user:Mutex<HashMap<String, String>>,
} 

#[derive(Debug, Default)]
pub struct UserInfo{
    // registration
    pub user_name : String,
    pub y1: BigUint,
    pub y2: BigUint,
    // authorization
    pub r1: BigUint,
    pub r2: BigUint,
    //verification
    pub c:BigUint,
    pub s:BigUint,
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl{
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {

        println!("Processing Register : {:?}", request);

        let request = request.into_inner();
        let user_name= request.user;
        // let y1 = BigUint::from_bytes_be(&request.y1);
        // let y2 = BigUint::from_bytes_be(&request.y2);


        let mut user_info = UserInfo::default();
        user_info.user_name = user_name.clone();
        user_info.y1 = BigUint::from_bytes_be(&request.y1);
        user_info.y2 = BigUint::from_bytes_be(&request.y2);

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(user_name, user_info);
        
        Ok(Response::new(RegisterResponse{}))
    }

    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status>{
        println!("Processing Register: {:?}", request);

        let request = request.into_inner();
        let user_name =  request.user;
        let mut user_info_hashmap = &mut self.user_info.lock().unwrap();

        if let Some(user_info) = user_info_hashmap.get_mut(&user_name){
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);


            // constant 
            let (_, _, _,q) = ZKP::get_constants();
            let c = ZKP::generate_random_less_than(&q);
            // let auth_id = "skdjfsk".to_string();
            let auth_id = ZKP::generate_random_stirng_below(12);

            user_info.c  = c.clone();

            let mut auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(),user_name);

            Ok(Response::new(AuthenticationChallengeResponse {auth_id,c: c.to_bytes_be()}))
        }   else{
            return Err(Status::new(Code::NotFound, format!("User : {} not found in database", user_name)))
        }


        
    }

    async fn verify_authentication(&self, request:Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status>{

        println!("Processing Verification Request : {:?}", request);

        let request = request.into_inner();
        let auth_id  = request.auth_id;
         
        let mut auth_id_to_user_hashmap =  &mut self.auth_id_to_user.lock().unwrap();

        if let Some(user_name) = auth_id_to_user_hashmap.get_mut(&auth_id){
            let mut  user_info_hashmap = &mut self.user_info.lock().unwrap();
            let user_info = user_info_hashmap.get_mut(user_name).expect("auth id not found in hashmap");

            let s =request.s;

            let (alpha, beta, p, q) = ZKP::get_constants();
            let zkp = ZKP::new(alpha, beta, p, q);
 
            let verification = zkp.verify(&user_info.r1, &user_info.r2, &user_info.y1, &user_info.y2, &BigUint::from_bytes_be(&s) , &user_info.c);
            
            if verification {
                
                let session_id = ZKP::generate_random_stirng_below(12);
                
                Ok(Response::new(AuthenticationAnswerResponse{session_id}))
            } else{
                Err(Status::new(Code::PermissionDenied, format!("AuthId: {} send a bad solution to the challenge", auth_id)))
            }
        }else{
            // Err(Status::new(Code::NotFound, format!("AuthId: {} not found in database", auth_id)))
            Err(Status::new(Code::NotFound, format!("AuthId: {} not found in database", auth_id)))
        }
        // todo!()
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