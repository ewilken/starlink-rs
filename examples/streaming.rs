use async_stream::stream;
use std::time::Duration;
use tokio::time::sleep;

use starlink::proto::space_x::api::device::{
    device_client::DeviceClient,
    request,
    to_device,
    GetStatusRequest,
    Request,
    ToDevice,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DeviceClient::connect("http://192.168.100.1:9200").await?;

    let request_stream = stream! {
        loop {
            yield ToDevice {
                message: Some(to_device::Message::Request(Request {
                    id: None,
                    epoch_id: None,
                    target_id: None,
                    request: Some(request::Request::GetStatus(GetStatusRequest {})),
                })),
            };

            sleep(Duration::from_secs(1)).await;
        }
    };

    let request = tonic::Request::new(request_stream);

    let mut response_stream = client.stream(request).await?.into_inner();

    while let Some(message) = response_stream.message().await? {
        dbg!(message);
    }

    Ok(())
}
