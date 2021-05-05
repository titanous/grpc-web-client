use std::time::Duration;

use crate::echo_server::{Echo, EchoServer};
use crate::greeter_server::{Greeter, GreeterServer};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

tonic::include_proto!("helloworld");
tonic::include_proto!("grpc.examples.echo");

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

type ResponseStream = ReceiverStream<Result<EchoResponse, Status>>;
#[derive(Default)]
pub struct MyEcho;

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }

    type ServerStreamingEchoStream = ResponseStream;

    async fn server_streaming_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<Self::ServerStreamingEchoStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        let message = request.into_inner().message;

        tokio::spawn(async move {
            tx.send(Ok(EchoResponse {
                message: message.clone(),
            }))
            .await
            .unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
            tx.send(Ok(EchoResponse { message })).await.unwrap();
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn client_streaming_echo(
        &self,
        _: Request<tonic::Streaming<EchoRequest>>,
    ) -> Result<Response<EchoResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    type BidirectionalStreamingEchoStream = ResponseStream;

    async fn bidirectional_streaming_echo(
        &self,
        _: Request<tonic::Streaming<EchoRequest>>,
    ) -> Result<Response<Self::BidirectionalStreamingEchoStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let greeter = MyGreeter::default();
    let echo = MyEcho::default();

    let config = tonic_web::config().allow_all_origins();

    Server::builder()
        .accept_http1(true)
        .add_service(config.enable(GreeterServer::new(greeter)))
        .add_service(config.enable(EchoServer::new(echo)))
        .serve("127.0.0.1:8080".parse().unwrap())
        .await?;

    Ok(())
}
