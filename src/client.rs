use login::login_client::LoginClient;
use login::LoginRequest;

pub mod login {
    tonic::include_proto!("login");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LoginClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        LoginRequest {
            from_addr: "123456".to_owned(),
            to_addr: "654321".to_owned(),
            amount: 22
        }
    );

    let response = client.send_login(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
