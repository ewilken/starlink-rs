pub mod proto {
    pub mod space_x {
        pub mod api {
            pub mod status {
                tonic::include_proto!("space_x.api.status");
            }

            pub mod device {
                tonic::include_proto!("space_x.api.device");
            }
        }
    }
}
