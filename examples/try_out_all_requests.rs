use starlink::proto::space_x::api::device::{
    device_client::DeviceClient,
    request,
    AuthenticateRequest,
    DishEmcRequest,
    DishGetContextRequest,
    DishGetObstructionMapRequest,
    DishStowRequest,
    EnableFlowRequest,
    FactoryResetRequest,
    FuseRequest,
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
    RestartControlRequest,
    SetSkuRequest,
    SetTrustedKeysRequest,
    SignedData,
    SpeedTestRequest,
    TransceiverGetStatusRequest,
    TransceiverGetTelemetryRequest,
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
// DishGetContextRequest            - previously working, now permission denied
// DishGetObstructionMapRequest     - working
// DishStowRequest                  -
// EnableFlowRequest                -
// FactoryResetRequest              -
// FuseRequest                      - unimplemented
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
// RestartControlRequest            - unimplemented
// SetSkuRequest                    -
// SetTrustedKeysRequest            -
// SignedData                       -
// SpeedTestRequest                 - unimplemented
// TransceiverGetStatusRequest      - unimplemented
// TransceiverGetTelemetryRequest   - unimplemented
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
    //     request: Some(request::Request::DishGetObstructionMap(DishGetObstructionMapRequest {})),
    // });
    // let response = client.handle(request).await?;
    // dbg!(response);

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

    let request = tonic::Request::new(Request {
        id: None,
        epoch_id: None,
        target_id: None,
        request: Some(request::Request::GetStatus(GetStatusRequest {})),
    });
    let response = client.handle(request).await?;
    dbg!(response);

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
    //                 "2bc83694-2dec-48c8-9061-88b86cdd5d89.uterm.release",
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
    //                 765906,
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
    //         9.0,
    //     ),
    //     seconds_to_first_nonempty_slot: None,
    //     pop_ping_drop_rate: None,
    //     downlink_throughput_bps: Some(
    //         176773.14,
    //     ),
    //     uplink_throughput_bps: Some(
    //         102408.125,
    //     ),
    //     pop_ping_latency_ms: Some(
    //         37.52381,
    //     ),
    //     obstruction_stats: Some(
    //         DishObstructionStats {
    //             currently_obstructed: None,
    //             fraction_obstructed: Some(
    //                 0.00081779656,
    //             ),
    //             last_24h_obstructed_s: Some(
    //                 15.0,
    //             ),
    //             valid_s: Some(
    //                 765202.0,
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
    //                 0.006051556,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //             ],
    //             wedge_abs_fraction_obstructed: [
    //                 0.000026709631,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //                 0.00033165555,
    //                 0.0,
    //                 0.0,
    //                 0.0,
    //             ],
    //         },
    //     ),
    //     stow_requested: None,
    // },

    Ok(())
}
