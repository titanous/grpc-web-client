use std::convert::TryInto;

tonic::include_proto!("helloworld");
tonic::include_proto!("grpc.examples.echo");

wasm_bindgen_test_configure!(run_in_browser);
use js_sys::Date;

use grpc_web_client::Client;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn hello_world() {
    let uri = "http://127.0.0.1:8080".try_into().expect("to be valid Uri");
    let client = Client::new(&uri).expect("to create Client");
    let mut client = greeter_client::GreeterClient::new(client);

    let request = tonic::Request::new(HelloRequest {
        name: "WebTonic".into(),
    });

    let response = client.say_hello(request).await.unwrap().into_inner();
    assert_eq!(response.message, "Hello WebTonic!");
}

#[wasm_bindgen_test]
async fn echo_unary() {
    let uri = "http://127.0.0.1:8080".try_into().expect("to be valid Uri");
    let client = Client::new(&uri).expect("to create Client");
    let mut client = echo_client::EchoClient::new(client);

    let request = tonic::Request::new(EchoRequest {
        message: "Echo Test".to_string(),
    });

    let response = client.unary_echo(request).await.unwrap().into_inner();
    assert_eq!(response.message, "Echo Test");
}

#[wasm_bindgen_test]
async fn echo_server_stream() {
    let uri = "http://127.0.0.1:8080".try_into().expect("to be valid Uri");
    let client = Client::new(&uri).expect("to create Client");
    let mut client = echo_client::EchoClient::new(client);

    let request = tonic::Request::new(EchoRequest {
        message: "Echo Test".to_string(),
    });

    let mut response = client
        .server_streaming_echo(request)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        response.message().await.unwrap().unwrap().message,
        "Echo Test"
    );
    let before_recv = Date::now();

    assert_eq!(
        response.message().await.unwrap().unwrap().message,
        "Echo Test"
    );

    let after_recv = Date::now();
    assert!(after_recv - before_recv > 100.0);
}
