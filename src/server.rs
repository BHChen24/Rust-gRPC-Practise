use std::error::Error;

use tonic::transport::Server;
use tonic::{self, Response};
use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordRespond};

pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService {}

#[tonic::async_trait]
impl Recorder for RecorderService {
    async fn send_message(&self, req: tonic::Request<RecordRequest>) -> Result<tonic::Response<RecordRespond>, tonic::Status> {
        println!("request: {:#?}", req);
        let reqst = req.into_inner();
        let res = RecordRespond {
            successful: true,
            message: format!("User {} is {} old", reqst.user_name, reqst.user_age),
        };
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let addr= "[::1]:50050".parse()?; 
    let recorder = RecorderService::default();
    println!("Recorder is listening: {}", addr);

    Server::builder().add_service(RecorderServer::new(recorder)).serve(addr).await?;

    Ok(())
}
