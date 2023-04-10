use tonic::{transport::Server, Request, Response, Status};

use login::login_server::{Login, LoginServer};
use login::{LoginRequest, LoginResponse};

pub mod login {
    tonic::include_proto!("login");
}

#[derive(Debug, Default)]
pub struct LoginSevice {}

#[tonic::async_trait]
impl Login for LoginSevice {
    async fn send_login(
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
    let btc_service = LoginSevice::default();
    println!("{}",addr);
    Server::builder()
        .add_service(LoginServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}