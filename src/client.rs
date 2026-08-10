use std::io::stdin;
use num_bigint::BigUint;

pub mod zkp_auth{
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_client::AuthClient, RegisterRequest , AuthenticationChallengeRequest};
use zero_knowledge_proofs::ZKP;

use crate::zkp_auth::AuthenticationAnswerRequest;

// use crate::zkp_auth::AuthenticationChallengeRequest;

// use crate::zkp_auth::RegisterRequest;


#[tokio::main]
async fn main(){

    let mut buf = String::new();
    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP::new(alpha.clone(), beta.clone(), p.clone(), q.clone());

    let mut client = AuthClient::connect("http://127.0.0.1:50051").await.expect("could not connect to the server");
    println!("Connected to the server");

    println!("Please provide username:");

    buf.clear();

    stdin().read_line(&mut buf).expect("Could not get the unsername from stdin");
    let username = buf.trim().to_string();

    buf.clear();

    println!("Please provide password:");

    stdin().read_line(&mut buf).expect("Could no the the passowrd from stdin ");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let y1 = ZKP::exponentiate(&alpha, &password, &p.clone());
    let y2 = ZKP::exponentiate(&beta, &password, &p.clone());

    let request = RegisterRequest{
        user: username.clone(),
        y1:y1.to_bytes_be(), 
        y2:y2.to_bytes_be(), 
    };

    println!("Please provide the password (to login): ");buf.clear();
    stdin().read_line(&mut buf).expect("Could no the the passowrd from stdin ");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());

    let _response = client.register(request).await.expect("could not  register");
    println!("{:?}", _response);    
    
    let k = ZKP::generate_random_less_than(&q);
    let r1 = ZKP::exponentiate(&alpha, &k, &p.clone());
    let r2 = ZKP::exponentiate(&beta, &k, &p.clone());

    let request = AuthenticationChallengeRequest{
        user: username,
        r1:r1.to_bytes_be(), 
        r2:r2.to_bytes_be(), 
    };

    let _response = client.create_authentication_challenge(request).await.expect("could not  register").into_inner();
    println!("{:?}", _response);  

    let auth_id = _response.auth_id;
    let c = BigUint::from_bytes_be( &_response.c);
    let s = zkp.solve(&k, &c, &password);


    let request = AuthenticationAnswerRequest{
        auth_id,
        s: s.to_bytes_be()
    };

    let response = client.verify_authentication(request).await.expect("Could not verify authentication in server").into_inner();
    println!("You logged in !!! Session_id is {}", response.session_id);
    
}