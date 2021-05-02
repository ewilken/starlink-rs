use starlink::proto::space_x::api::device::{device_client::DeviceClient, GetStatusRequest, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DeviceClient::connect("http://192.168.100.1:9200").await?;

    let request = tonic::Request::new(Request {
        id: None,
        epoch_id: None,
        target_id: None,
        signed_request: None,
        get_next_id: None,
        authenticate: None,
        factory_reset: None,
        get_history: None,
        get_log: None,
        get_ping: None,
        get_device_info: None,
        get_status: Some(GetStatusRequest {}),
        reboot: None,
        set_trusted_keys: None,
        speed_test: None,
        dish_stow: None,
        wifi_get_clients: None,
        wifi_set_config: None,
        wifi_setup: None,
    });

    let response = client.handle(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())
}
