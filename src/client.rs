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
            user_id: "123456".to_owned(),
            user_pw: "654321".to_owned(),
            user_name: "654321".to_owned(),
        }
    );

    let response = client.logins(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}