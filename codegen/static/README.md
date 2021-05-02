Retrieving the protoset from dish server reflection:

    grpcurl -plaintext -protoset-out proto/dish.protoset 192.168.100.1:9200 describe SpaceX.API.Device.Device
