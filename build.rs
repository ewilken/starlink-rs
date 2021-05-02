fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .format(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &[
                "proto/spacex/api/common/status/status.proto",
                "proto/spacex/api/device/command.proto",
                "proto/spacex/api/device/common.proto",
                "proto/spacex/api/device/device.proto",
                "proto/spacex/api/device/dish.proto",
                "proto/spacex/api/device/service.proto",
                "proto/spacex/api/device/wifi_config.proto",
                "proto/spacex/api/device/wifi.proto",
            ],
            &["proto/"],
        )?;

    Ok(())
}
