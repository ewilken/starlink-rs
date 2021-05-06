# starlink-rs

[![CI](https://github.com/ewilken/starlink-rs/workflows/CI/badge.svg)](https://github.com/ewilken/starlink-rs/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/starlink.svg)](https://crates.io/crates/starlink)
[![docs.rs](https://docs.rs/starlink/badge.svg)](https://docs.rs/starlink)
[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/ewilken/starlink-rs)

Rust client implementation to the gRPC endpoint exposed by the SpaceX Starlink user terminal.

**Disclaimer:** If you're Elon or another SpaceX-affiliated authority and don't want this to exist, please just contact me.

## Background

The Starlink dish exposes an unauthenticated gRPC HTTP/2 server on its local network under `192.168.100.1:9200` that allows for [server reflection](https://github.com/grpc/grpc/blob/master/doc/server-reflection.md). This contains the (probably still flawed) Protobuf definitions reversed out of it as well as a Rust client implementation being able to talk to the dish, for science.

The dish exposes two methods (as far as I could tell); one for request/response and one for streams:

```protobuf
service Device {
    rpc Handle (.SpaceX.API.Device.Request) returns (.SpaceX.API.Device.Response) {}
    rpc Stream (stream .SpaceX.API.Device.ToDevice) returns (stream .SpaceX.API.Device.FromDevice) {}
}
```

## Example Usage

Working examples for request/response and streaming communication are in the [`/examples`](https://github.com/ewilken/starlink-rs/tree/main/examples) directory.

### Request / Response

```rust
use starlink::proto::space_x::api::device::{device_client::DeviceClient, request, GetStatusRequest, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DeviceClient::connect("http://192.168.100.1:9200").await?;

    let request = tonic::Request::new(Request {
        id: None,
        epoch_id: None,
        target_id: None,
        request: Some(request::Request::GetStatus(GetStatusRequest {})),
    });

    let response = client.handle(request).await?;

    dbg!(response);

    Ok(())
}
```

```
cargo run --example request_response
```

Prints something like:

```
Response {
    metadata: MetadataMap {
        headers: {
            "content-type": "application/grpc",
            "grpc-status": "0",
            "grpc-message": "",
        },
    },
    message: Response {
        id: None,
        status: None,
        response: Some(
            DishGetStatus(
                DishGetStatusResponse {
                    device_info: Some(
                        DeviceInfo {
                            id: Some(
                                "<my-ID>",
                            ),
                            hardware_version: Some(
                                "rev1_pre_production",
                            ),
                            software_version: Some(
                                "1f86ec34-34ea-4e7a-9758-3842e72422fb.release",
                            ),
                            country_code: Some(
                                "DE",
                            ),
                            utc_offset_s: Some(
                                1,
                            ),
                        },
                    ),
                    device_state: Some(
                        DeviceState {
                            uptime_s: Some(
                                26115,
                            ),
                        },
                    ),
                    state: Some(
                        Connected,
                    ),
                    alerts: Some(
                        DishAlerts {
                            motors_stuck: None,
                            thermal_throttle: None,
                            thermal_shutdown: None,
                            mast_not_near_vertical: None,
                            unexpected_location: None,
                            slow_ethernet_speeds: None,
                        },
                    ),
                    snr: Some(
                        6.0,
                    ),
                    seconds_to_first_nonempty_slot: None,
                    pop_ping_drop_rate: None,
                    downlink_throughput_bps: Some(
                        8584784.0,
                    ),
                    uplink_throughput_bps: Some(
                        311510.97,
                    ),
                    pop_ping_latency_ms: Some(
                        38.857143,
                    ),
                    obstruction_stats: Some(
                        DishObstructionStats {
                            currently_obstructed: None,
                            fraction_obstructed: Some(
                                0.0010516815,
                            ),
                            last_24h_obstructed_s: Some(
                                72.0,
                            ),
                            valid_s: Some(
                                21260.67,
                            ),
                            wedge_fraction_obstructed: [
                                1.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                            ],
                            wedge_abs_fraction_obstructed: [
                                0.0010516815,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                            ],
                        },
                    ),
                },
            ),
        ),
    },
}
```

### Streaming

```rust
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
```

```
cargo run --example streaming
```

## Development

Protobuf codegen is handled by the `codegen` crate in the workspace. Generated Protobuf files are checked in. To run the code generation, do:

    cargo run --package starlink-codegen

## License

`starlink-rs` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
