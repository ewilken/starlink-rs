use starlink::proto::space_x::api::device::{
    device_client::DeviceClient,
    request,
    AuthenticateRequest,
    DishEmcRequest,
    DishGetContextRequest,
    DishStowRequest,
    EnableFlowRequest,
    FactoryResetRequest,
    GetDeviceInfoRequest,
    GetHeapDumpRequest,
    GetHistoryRequest,
    GetLocationRequest,
    GetLogRequest,
    GetNetworkInterfacesRequest,
    GetNextIdRequest,
    GetPingRequest,
    GetStatusRequest,
    PingHostRequest,
    RebootRequest,
    Request,
    SetSkuRequest,
    SetTrustedKeysRequest,
    SignedData,
    SpeedTestRequest,
    TransceiverGetStatusRequest,
    TransceiverIfLoopbackTestRequest,
    UpdateRequest,
    WifiGetClientsRequest,
    WifiGetConfigRequest,
    WifiGetDiagnosticsRequest,
    WifiGetPingMetricsRequest,
    WifiSetConfigRequest,
    WifiSetupRequest,
};

// AuthenticateRequest              -
// DishEmcRequest                   -
// DishGetContextRequest            - working
// DishStowRequest                  -
// EnableFlowRequest                -
// FactoryResetRequest              -
// GetDeviceInfoRequest             - working
// GetHeapDumpRequest               - unimplemented
// GetHistoryRequest                - working
// GetLocationRequest               - permission denied
// GetLogRequest                    - unimplemented
// GetNetworkInterfacesRequest      - unimplemented
// GetNextIdRequest                 - unimplemented
// GetPingRequest                   - unimplemented
// GetStatusRequest                 - working
// PingHostRequest                  - unimplemented
// RebootRequest                    - working
// Request                          -
// SetSkuRequest                    -
// SetTrustedKeysRequest            -
// SignedData                       -
// SpeedTestRequest                 - unimplemented
// TransceiverGetStatusRequest      - unimplemented
// TransceiverIfLoopbackTestRequest -
// UpdateRequest                    - unimplemented
// WifiGetClientsRequest            - unimplemented
// WifiGetConfigRequest             - unimplemented
// WifiGetDiagnosticsRequest        - unimplemented
// WifiGetPingMetricsRequest        - unimplemented
// WifiSetConfigRequest             - unimplemented
// WifiSetupRequest                 - unimplemented

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DeviceClient::connect("http://192.168.100.1:9200").await?;

    // let request = tonic::Request::new(Request {
    //     id: None,
    //     epoch_id: None,
    //     target_id: None,
    //     request: Some(request::Request::GetDeviceInfo(GetDeviceInfoRequest {})),
    // });

    // let response = client.handle(request).await?;

    // dbg!(response);

    // GetDeviceInfoResponse {
    //     device_info: Some(
    //         DeviceInfo {
    //             id: Some(
    //                 "<my-ID>",
    //             ),
    //             hardware_version: Some(
    //                 "rev1_pre_production",
    //             ),
    //             software_version: Some(
    //                 "1f86ec34-34ea-4e7a-9758-3842e72422fb.release",
    //             ),
    //             country_code: Some(
    //                 "DE",
    //             ),
    //             utc_offset_s: Some(
    //                 1,
    //             ),
    //         },
    //     ),
    // }

    // let request = tonic::Request::new(Request {
    //     id: None,
    //     epoch_id: None,
    //     target_id: None,
    //     request: Some(request::Request::DishGetContext(DishGetContextRequest {})),
    // });

    // let response = client.handle(request).await?;

    // dbg!(response);

    // DishGetContextResponse {
    //     device_info: Some(
    //         DeviceInfo {
    //             id: Some(
    //                 "<my-ID>",
    //             ),
    //             hardware_version: Some(
    //                 "rev1_pre_production",
    //             ),
    //             software_version: Some(
    //                 "1f86ec34-34ea-4e7a-9758-3842e72422fb.release",
    //             ),
    //             country_code: Some(
    //                 "DE",
    //             ),
    //             utc_offset_s: Some(
    //                 1,
    //             ),
    //         },
    //     ),
    //     device_state: Some(
    //         DeviceState {
    //             uptime_s: Some(
    //                 25298,
    //             ),
    //         },
    //     ),
    //     obstruction_fraction: Some(
    //         0.001093006,
    //     ),
    //     obstruction_valid_s: Some(
    //         20723.25,
    //     ),
    //     cell_id: Some(
    //         314900,
    //     ),
    //     pop_rack_id: Some(
    //         18,
    //     ),
    //     seconds_to_slot_end: Some(
    //         12.513584,
    //     ),
    // }

    // let request = tonic::Request::new(Request {
    //     id: None,
    //     epoch_id: None,
    //     target_id: None,
    //     request: Some(request::Request::GetStatus(GetStatusRequest {})),
    // });

    // let response = client.handle(request).await?;

    // dbg!(response);

    // DishGetStatusResponse {
    //     device_info: Some(
    //         DeviceInfo {
    //             id: Some(
    //                 "<my-ID>",
    //             ),
    //             hardware_version: Some(
    //                 "rev1_pre_production",
    //             ),
    //             software_version: Some(
    //                 "1f86ec34-34ea-4e7a-9758-3842e72422fb.release",
    //             ),
    //             country_code: Some(
    //                 "DE",
    //             ),
    //             utc_offset_s: Some(
    //                 1,
    //             ),
    //         },
    //     ),
    //     device_state: Some(
    //         DeviceState {
    //             uptime_s: Some(
    //                 26115,
    //             ),
    //         },
    //     ),
    //     state: Some(
    //         Connected,
    //     ),
    //     alerts: Some(
    //         DishAlerts {
    //             motors_stuck: None,
    //             thermal_throttle: None,
    //             thermal_shutdown: None,
    //             mast_not_near_vertical: None,
    //             unexpected_location: None,
    //             slow_ethernet_speeds: None,
    //         },
    //     ),
    //     snr: Some(
    //         6.0,
    //     ),
    //     seconds_to_first_nonempty_slot: None,
    //     pop_ping_drop_rate: None,
    //     downlink_throughput_bps: Some(
    //         8584784.0,
    //     ),
    //     uplink_throughput_bps: Some(
    //         311510.97,
    //     ),
    //     pop_ping_latency_ms: Some(
    //         38.857143,
    //     ),
    //     obstruction_stats: Some(
    //         DishObstructionStats {
    //             currently_obstructed: None,
    //             fraction_obstructed: Some(
    //                 0.0010516815,
    //             ),
    //             last_24h_obstructed_s: Some(
    //                 72.0,
    //             ),
    //             valid_s: Some(
    //                 21260.67,
    //             ),
    //             wedge_fraction_obstructed: [
    //                 1.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //             ],
    //             wedge_abs_fraction_obstructed: [
    //                 0.0010516815,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //             ],
    //         },
    //     ),
    // }

    Ok(())
}
