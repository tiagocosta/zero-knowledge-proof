pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use tonic::{transport::Server, Request, Response, Status};
use zkp_auth::{auth_server::{Auth, AuthServer}, AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest, RegisterResponse};

#[derive(Debug, Default)]
struct AuthImpl {}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(&self, request: Request<RegisterRequest>) ->  Result<Response<RegisterResponse>, Status> {
        todo!()
    }

    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) ->  Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }

    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) ->  Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:50051");
    println!("Running the server on {}", addr);
    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("invalid address"))
        .await
        .unwrap();
}
