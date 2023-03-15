fn main() {
    println!("Hello, world!");
}
// tonic, prost, tokio 라이브러리를 가져온다.
use tonic::transport::Channel;

use tokio::sync::mpsc;

// gRPC 요청에 필요한 프로토콜 버퍼 파일을 가져온다.
// 예를 들어, 파일 이름이 my_service.proto 이면 my_service_proto 라이브러리를 생성할 수 있다.
use my_service_proto::{my_service_client::MyServiceClient, MyRequest, MyResponse};

// actix 웹 프레임워크에서 요청 핸들러를 정의한다.
async fn handle_request() -> HttpResponse {
    // tonic gRPC 채널을 생성한다.
    let channel = Channel::from_static("http://grpc.server.com:50051")
        .connect()
        .await
        .unwrap();

    // MyServiceClient를 사용하여 gRPC 클라이언트를 생성한다.
    let mut client = MyServiceClient::new(channel);

    // gRPC 요청 메시지를 생성한다.
    let request = MyRequest {
        // 요청에 필요한 필드를 채운다.
        field1: "value1".to_string(),
        field2: 42,
    };

    // gRPC 요청을 보내고 결과를 받는다.
    let response = client
        .my_grpc_method(request)
        .await
        .unwrap()
        .into_inner();

    // 받은 응답을 처리한다.
    // 예를 들어, 응답에 포함된 필드를 HTTP 응답으로 변환한다.
    HttpResponse::Ok()
        .body(response.field3)
}