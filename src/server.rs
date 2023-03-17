use tonic::{transport::Server, Request, Response, Status};

use login::login_server::{Login, LoginServer};
use login::{LoginResponse, LoginRequest};

pub mod login {
    tonic::include_proto!("login");
}

#[derive(Debug, Default)]
pub struct LoginService {}

#[tonic::async_trait]
impl Login for LoginService {
    async fn logins(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = LoginResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = LoginService::default();

    Server::builder()
        .add_service(LoginServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}