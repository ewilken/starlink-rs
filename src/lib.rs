pub mod proto {
    pub mod space_x {
        pub mod api {
            pub mod status {
                tonic::include_proto!("space_x.api.status");
            }

            pub mod device {
                tonic::include_proto!("space_x.api.device");
                pub mod services {
                    pub mod unlock {
                        tonic::include_proto!("space_x.api.device.services.unlock");
                    }
                }
            }

            pub mod satellites {
                pub mod network {
                    tonic::include_proto!("space_x.api.satellites.network");
                }
            }

            pub mod telemetron {
                pub mod public {
                    pub mod common {
                        tonic::include_proto!("space_x.api.telemetron.public.common");
                    }
                }
            }
        }
    }
}
