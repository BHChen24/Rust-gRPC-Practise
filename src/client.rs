use std::error::Error;

use records::recorder_client::RecorderClient;
use records::RecordRequest;
use tonic::Request;

pub mod records {
    tonic::include_proto!("records");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = RecorderClient::connect("http://[::1]:50050").await?;
    let req = Request::new(RecordRequest {
        user_name: "myName".to_string(),
        user_age: 28,
    });
    let res = client.send_message(req).await?;
    println!("{:#?}", res);
    println!("Metadata: \n{:#?}", res.metadata());
    println!("Only message: \n{:#?}", res.get_ref().message);
    Ok(())
}
