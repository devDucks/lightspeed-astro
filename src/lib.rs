pub mod props {
    include!(concat!(env!("OUT_DIR"), "/lightspeed.props.rs"));
}

pub mod devices {
    include!(concat!(env!("OUT_DIR"), "/lightspeed.devices.rs"));
    pub mod actions {
        include!(concat!(env!("OUT_DIR"), "/lightspeed.devices.actions.rs"));
    }
}

pub mod request {
    include!(concat!(env!("OUT_DIR"), "/lightspeed.request.rs"));
}

pub mod response {
    include!(concat!(env!("OUT_DIR"), "/lightspeed.response.rs"));
}

pub mod server {
    include!(concat!(env!("OUT_DIR"), "/lightspeed.server.rs"));
}

pub mod proto {
    pub const FD_DESCRIPTOR_SET: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/lightspeed.bin"));
}
